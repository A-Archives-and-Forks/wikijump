/*
 * endpoints/macros.rs
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

macro_rules! parse {
    ($params_method:ident; $params:expr, $error_type:ident $(,)?) => {
        $params.$params_method().or_raise(|| Error::new(
            "failed to read input JSON",
            ErrorType::$error_type,
        ))?
    };

    ($params:expr, $error_type:ident $(,)?) => {
        parse!(parse; $params, $error_type)
    };
}

macro_rules! parse_one {
    ($params:expr, $error_type:ident $(,)?) => {
        parse!(one; $params, $error_type)
    };
}
