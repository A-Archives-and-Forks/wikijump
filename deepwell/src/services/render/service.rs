/*
 * services/render/service.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2019-2026 Wikijump Team
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
use crate::services::text_block::{MIME_HTML, TextBlock, mime_for_language};
use crate::services::{TextBlockService, TextService};
use ftml::{prelude::*, tree::CodeBlock};
use tokio::time::timeout;

#[derive(Debug)]
pub struct RenderService;

impl RenderService {
    pub async fn render(
        ctx: &ServiceContext<'_>,
        wikitext: String,
        page_info: &PageInfo<'_>,
        settings: &WikitextSettings,
    ) -> Result<RenderOutput> {
        Self::render_inner(ctx, wikitext, page_info, settings, None).await
    }

    pub async fn render_page(
        ctx: &ServiceContext<'_>,
        wikitext: String,
        page_info: &PageInfo<'_>,
        layout: Layout,
        page_id: i64,
    ) -> Result<RenderOutput> {
        let settings = WikitextSettings::from_mode(WikitextMode::Page, layout);
        Self::render_inner(ctx, wikitext, page_info, &settings, Some(page_id)).await
    }

    async fn render_inner(
        ctx: &ServiceContext<'_>,
        mut wikitext: String,
        page_info: &PageInfo<'_>,
        settings: &WikitextSettings,
        page_id: Option<i64>,
    ) -> Result<RenderOutput> {
        let compiled_generator = FTML_VERSION.clone();
        let config = ctx.config();

        // We isolate the actual tasks for rendering,
        // allowing us to time it out if it takes too long.
        //
        // The preprocess step has to be distinct for borrowing reasons,
        // since we want to do the processing for non-ftml work
        // outside the timeout guards.

        let tokens = timeout(config.preprocess_timeout, async {
            // TODO include
            ftml::preprocess(&mut wikitext);
            ftml::tokenize(&wikitext)
        })
        .await
        // Not using Error::from() because timeouts could occur in other places,
        // and this error variant is not specific to all timeouts.
        .map_err(|_| Error::RenderTimeout)?;

        let (tree, html_output, errors) = timeout(config.render_timeout, async {
            let result = ftml::parse(&tokens, page_info, settings);
            let (tree, errors) = result.into();
            let html_output = HtmlRender.render(&tree, page_info, settings);
            (tree, html_output, errors)
        })
        .await
        // As above, just doing the timeout error conversion here.
        .map_err(|_| Error::RenderTimeout)?;

        // Insert compiled HTML into text table
        let compiled_hash = TextService::create(ctx, html_output.body.clone()).await?;

        // Set up the hosted text blocks
        //
        // This only applies for published pages, in any other
        // rendering context and we should skip this step.

        if let Some(page_id) = page_id {
            // It's possible to render a page without doing text blocks
            // (e.g. special pages), but all cases where text blocks
            // are done are pages.
            debug_assert_eq!(settings.mode, WikitextMode::Page);

            // [[html]]
            let html_blocks: Vec<TextBlock> = tree
                .html_blocks
                .iter()
                .map(|html| TextBlock {
                    text: html,
                    text_type: None,
                    mime: MIME_HTML,
                    name: None,
                })
                .collect();

            TextBlockService::add_blocks(ctx, page_id, TextBlockType::Html, &html_blocks)
                .await?;

            // [[code]]
            let code_blocks: Vec<TextBlock> = tree
                .code_blocks
                .iter()
                .map(
                    |CodeBlock {
                         contents,
                         language,
                         name,
                     }| TextBlock {
                        text: contents,
                        text_type: language.as_deref(),
                        mime: mime_for_language(language),
                        name: name.as_deref(),
                    },
                )
                .collect();

            TextBlockService::add_blocks(ctx, page_id, TextBlockType::Code, &code_blocks)
                .await?;
        }

        // Build and return
        Ok(RenderOutput {
            html_output,
            errors,
            compiled_hash,
            compiled_at: now(),
            compiled_generator,
        })
    }
}
