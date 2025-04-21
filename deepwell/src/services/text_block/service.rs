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

        // First, get the largest block index for this type.
        // This is needed for the step where we delete extraneous objects in S3.
        //
        // Consider a page which had 3 code blocks but now only has 2. Clearly,
        // we need to delete the last one (the other two will get overwritten
        // with the PutObject class). So we fetch the maximum block index and
        // delete everything from index blocks.len() through max_index.

        let txn = ctx.transaction();
        let max_index: usize = {
            let row: Option<i64> = TextBlockTable::find()
                .select_only()
                .column(text_block::Column::BlockIndex)
                .order_by_desc(text_block::Column::BlockIndex)
                .into_tuple()
                .one(txn)
                .await?;

            match row {
                None => 0,
                Some(index) => index.try_into().expect("Unable to convert to usize"),
            }
        };

        // As described above, we delete these extra blocks from S3.
        todo!();

        // Then, delete the blocks from the database.
        //
        // This doesn't require us to know which need to be kept
        // because we're just INSERTing over all of it.
        todo!();

        // Insert the new blocks into the database.
    }
}
