/*
 * services/text_block/service.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2019-2025 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

//! Manages the storage for hosted text blocks.
//!
//! This does _not_ have any methods for doing CRUD
//! on individual entries, since text blocks are only
//! updated when the underlying page is, which means
//! they all get replaced in one operation.
//!
//! Additionally, for fetching text blocks, this is
//! done by wws by directly accessing S3 itself, so
//! nothing here is needed.

use super::prelude::*;
use crate::models::sea_orm_active_enums::TextBlockType;
use crate::models::text_block::{
    self, Entity as TextBlockTable, Model as TextBlockModel,
};
use sea_orm::ActiveEnum;

#[derive(Debug)]
pub struct TextBlockService;

impl TextBlockService {
    /// Replaces the text blocks associated with this page with the ones given.
    ///
    /// This is to be run after ftml returns the lists of code and html blocks
    /// found in the source, which will replace the existing text block data
    /// to be replaced.
    pub async fn add_blocks(
        ctx: &ServiceContext<'_>,
        page_id: i64,
        block_type: TextBlockType,
        blocks: &[TextBlock<'_>],
    ) -> Result<()> {
        use std::ops::Add;

        info!(
            "Inserting {} text blocks for page ID {}",
            blocks.len(),
            page_id,
        );

        let txn = ctx.transaction();
        let bucket = ctx.s3_tblocks_bucket();
        let block_type_value = block_type.to_value();

        // Reuse this buffer for writing out S3 filenames.
        // They all take the format of "<PAGE ID>_<BLOCK TYPE>_<BLOCK INDEX>".
        // These are always 1-indexed, since that's how they're addressed via URL.
        // To give some examples of filenames, consider we have a page with ID
        // 12345 which has 2 code blocks and 3 html blocks. The objects in the bucket
        // would be:
        //
        // * "12345_code_1"
        // * "12345_code_2"
        // * "12345_html_1"
        // * "12345_html_2"
        // * "12345_html_3"

        let mut buffer = String::new();

        macro_rules! filename {
            ($index:expr) => {{
                buffer.clear();
                let index = $index;
                debug_assert_ne!(index, 0, "Text block indices must be 1-indexed!");
                str_write!(&mut buffer, "{page_id}_{block_type_value}_{index}");
                &buffer
            }};
        }

        // First, get the largest block index for this type.
        // This is needed for the step where we delete extraneous objects in S3.
        //
        // Consider a page which had 5 code blocks but now only has 2. Clearly,
        // we need to delete the last three (the other two will get overwritten
        // with the PutObject operation). So we fetch the maximum block index and
        // delete everything from index blocks.len() through max_index.
        //
        // The 1-indexing does not present a problem for the SQL query, since it
        // gets the maximum value, which is 1-indexed, thus representing the total
        // count of blocks presently kept.
        //
        // However, it does not present an issue for the block count, since if there are
        // 3 blocks being passed in, then the count is 3 and the range of indices must go
        // to 4 to be inclusive of 3. This is why we add one to max_index.

        let prev_max_index: i16 = TextBlockTable::find()
            .select_only()
            .column(text_block::Column::BlockIndex)
            .filter(
                Condition::all()
                    .add(text_block::Column::BlockType.eq(block_type))
                    .add(text_block::Column::PageId.eq(page_id)),
            )
            .order_by_desc(text_block::Column::BlockIndex)
            .limit(1)
            .into_tuple()
            .one(txn)
            .await?
            .unwrap_or(0);

        let max_index: i16 = blocks
            .len()
            .add(1)
            .try_into()
            .expect("Unable to fit block count in a i16");

        // As described above, we delete these extra blocks from S3.
        // If there are more or the same number of blocks now,
        // then this will do nothing.

        for index in max_index..prev_max_index {
            let index = index + 1;
            let filename = filename!(index);
            debug!("Deleting now-out-of-range S3 text block {filename}");
            bucket.delete_object(filename).await?;
        }

        // Upload the new text blocks to S3.
        // This also replaces the existing S3 objects,
        // which is why we don't have to delete everything above.
        //
        // While we're at it, we can also create the models to be
        // inserted to the database.

        let mut models = Vec::new();
        for (index, TextBlock { text, mime }) in blocks.iter().enumerate() {
            let index = index + 1;
            let filename = filename!(index);
            debug!("Uploading new S3 text block {filename} ({mime})");
            bucket
                .put_object_with_content_type(filename, text.as_bytes(), mime)
                .await?;

            let index: i16 = index
                .try_into()
                .expect("Unable to convert block index in a i16");

            models.push(text_block::ActiveModel {
                block_type: Set(block_type),
                page_id: Set(page_id),
                block_index: Set(index),
                block_name: Set(None), // TODO add block names
            });
        }

        // Then, delete the blocks from the database.
        //
        // This doesn't require us to know which need to be kept
        // because we're just INSERTing over all of it.

        let DeleteResult { rows_affected } = TextBlockTable::delete_many()
            .filter(
                Condition::all()
                    .add(text_block::Column::BlockType.eq(block_type))
                    .add(text_block::Column::PageId.eq(page_id)),
            )
            .exec(txn)
            .await?;

        debug_assert_eq!(
            rows_affected, max_index as u64,
            "Deleted row count do not match maximum block index",
        );

        // Finally, insert the batch of new text block rows, then return.
        TextBlockTable::insert_many(models).exec(txn).await?;
        Ok(())
    }
}
