## Audit Log

Wikijump supports a platform audit log, which is an append-only record of various mutation events. All entries are added to the `audit_log` table, where the data shape depends on the event type.

Note that the `audit_log` table is implemented using a variety of nullable columns. For a particular event type, some fields are nullable and the miscellaneous fields are given a particular meaning. Any non-nullable fields are set for all event types.

Additionally, for performance reasons, this table does not use foreign keys in Postgres itself, though the code must ensure this invariant regardless.

All event types take the form `[object].[operation]`, describing the data object being acted on and the kind of operation or event that has occurred.

This file will document all event types, describe their data, and explain when this auditing event is raised.

| Event Type        | Normal Columns                    | `extra_id_1`  | `extra_id_2`  | `extra_string_1`  | `extra_string_2`  | Notes |
|-------------------|-----------------------------------|---------------|---------------|-------------------|-------------------| ------|
| `user.create`     | `user_id`                         |               |               |                   |                   |       |
| `site.create`     | `site_id`                         |               |               |                   |                   |       |
| `page.create`     | `user_id`, `site_id`, `page_id`   | Revision ID   | Category ID   |                   |                   |       |
| `page.edit`       | `user_id`, `site_id`, `page_id`   | Revision ID   |               |                   |                   | The revision ID can be `NULL` if the edit did not result in a new revision being created. |
| `page.move`       | `user_id`, `site_id`, `page_id`   | Revision ID   |               | Old Page Slug     | New Page Slug     |       |
| `page.delete`     | `user_id`, `site_id`, `page_id`   | Revision ID   |               | Page Slug         |                   | The page slug is the value at the time of deletion. |
| `page.undelete`   | `user_id`, `site_id`, `page_id`   | Revision ID   | Category ID   | Page Slug         |                   | The page slug is the location the page is being restored to. |
