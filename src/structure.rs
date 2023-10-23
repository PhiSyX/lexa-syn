// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃                           __    _            ____  ______                 ┃
// ┃                    ____  / /_  (_)______  __/ __ \/ ____/                 ┃
// ┃                   / __ \/ __ \/ / ___/ / / / /_/ / /                      ┃
// ┃                  / /_/ / / / / (__  ) /_/ / _, _/ /___                    ┃
// ┃                 / .___/_/ /_/_/____/\__, /_/ |_|\____/                    ┃
// ┃                /_/                 /____/                                 ┃
// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ GNU General Public License v3.0+:                                         ┃
// ┃    see COPYING or https://www.gnu.org/licenses/gpl-3.0.txt                ┃
// ┃ SPDX-License-Identifier: GPL-3.0-or-later                                 ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃ This file is part of phisyRC.                                             ┃
// ┃                                                                           ┃
// ┃ phisyRC is free software: you can redistribute it and/or modify it under  ┃
// ┃ the terms of the GNU General Public License as published by the           ┃
// ┃ Free Software Foundation, either version 3 of the License, or (at your    ┃
// ┃ option) any later version.                                                ┃
// ┃                                                                           ┃
// ┃ phisyRC is distributed in the hope that it will be useful, but WITHOUT    ┃
// ┃ ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or     ┃
// ┃ FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for  ┃
// ┃ more details.                                                             ┃
// ┃                                                                           ┃
// ┃ You should have received a copy of the GNU General Public License along   ┃
// ┃ with phisyRC.                                                             ┃
// ┃                                                                           ┃
// ┃ If not, see <https://www.gnu.org/licenses/>.                              ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

// -------- //
// Fonction //
// -------- //

/// Cherche l'attribut passé en argument parmi la liste des attributs d'un
/// champ.
pub fn find_attr(field: &syn::ItemStruct, attr_name: impl AsRef<str>) -> Option<&syn::Attribute>
{
	field.attrs.iter().find(|attr| attr.path().is_ident(attr_name.as_ref()))
}
