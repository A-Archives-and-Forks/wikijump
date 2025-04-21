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

use super::prelude::*;
use crate::models::sea_orm_active_enums::TextBlockType;
use crate::models::text_block::{
    self, Entity as TextBlockTable, Model as TextBlockModel,
};
use sea_orm::ActiveEnum;

#[derive(Debug)]
pub struct TextBlockService;

impl TextBlockService {
    pub async fn add_blocks(
        ctx: &ServiceContext<'_>,
        page_id: i64,
        block_type: TextBlockType,
        blocks: &[TextBlock<'_>],
    ) -> Result<()> {
        info!(
            "Inserting {} text blocks for page ID {}",
            blocks.len(),
            page_id,
        );

        let txn = ctx.transaction();
        let bucket = ctx.s3_tblocks_bucket();
        let block_type_value = block_type.to_value();

        // Reuse this buffer for writing out S3 filenames.
        // They all take the format of "<BLOCK TYPE>_<PAGE ID>_<BLOCK INDEX>",
        // for instance "code_12345_2".

        let mut buffer = String::new();

        macro_rules! filename {
            ($index:expr) => {{
                buffer.clear();
                let index = $index;
                str_write!(&mut buffer, "{block_type_value}_{page_id}_{index}");
                &buffer
            }};
        }

        // First, get the largest block index for this type.
        // This is needed for the step where we delete extraneous objects in S3.
        //
        // Consider a page which had 3 code blocks but now only has 2. Clearly,
        // we need to delete the last one (the other two will get overwritten
        // with the PutObject class). So we fetch the maximum block index and
        // delete everything from index blocks.len() through max_index.

        let prev_max_index: i64 = TextBlockTable::find()
            .select_only()
            .column(text_block::Column::BlockIndex)
            .filter(
                Condition::all()
                    .add(text_block::Column::BlockType.eq(block_type))
                    .add(text_block::Column::PageId.eq(page_id)),
            )
            .order_by_desc(text_block::Column::BlockIndex)
            .into_tuple()
            .one(txn)
            .await?
            .unwrap_or(0);

        let max_index: i64 = blocks
            .len()
            .try_into()
            .expect("Unable to convert usize to i64");

        // As described above, we delete these extra blocks from S3.
        // If there are more or the same number of blocks now,
        // then this will do nothing.

        for block_index in max_index..prev_max_index {
            let filename = filename!(block_index);
            debug!("Deleting now-out-of-range S3 text block {filename}");
            bucket.delete_object(filename).await?;
        }

        // Upload the new text blocks to S3.
        // This also replaces the existing S3 objects,
        // which is why we don't have to delete everything above.

        for block in blocks {
            // TODO
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

        // Finally, insert the new blocks into the database.
        // TODO

        todo!()
    }
}
