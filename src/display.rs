use anyhow::Result;
use tabled::{
    Table,
    settings::{
        Alignment, Color, Modify, Style, Width,
        object::{Columns, Rows},
    },
};

use crate::{
    AliasEntry, BookmarkTableEntry, CategoryEntry, CleanedEntry, DomainEntry, DuplicateEntry,
    FunctionEntry, OrganizeSuggestion, PackageEntry,
};

pub fn display_aliases_table(aliases: Vec<AliasEntry>, use_colors: bool) -> Result<()> {
    let mut table = Table::new(&aliases);

    table.with(Style::rounded());

    if use_colors {
        table
            .with(
                Modify::new(Rows::first())
                    .with(Color::BG_BLUE)
                    .with(Color::FG_WHITE)
                    .with(Alignment::center()),
            )
            .with(
                Modify::new(Columns::new(0..1))
                    .with(Color::FG_CYAN)
                    .with(Width::wrap(20)),
            )
            .with(
                Modify::new(Columns::new(1..2))
                    .with(Color::FG_GREEN)
                    .with(Width::wrap(50)),
            )
            .with(
                Modify::new(Columns::new(2..3))
                    .with(Color::FG_YELLOW)
                    .with(Width::wrap(15)),
            );
    } else {
        table
            .with(Modify::new(Rows::first()).with(Alignment::center()))
            .with(Modify::new(Columns::new(0..1)).with(Width::wrap(20)))
            .with(Modify::new(Columns::new(1..2)).with(Width::wrap(50)))
            .with(Modify::new(Columns::new(2..3)).with(Width::wrap(15)));
    }

    println!("\n{}", table);

    Ok(())
}

pub fn display_functions_table(functions: Vec<FunctionEntry>, use_colors: bool) -> Result<()> {
    let mut table = Table::new(&functions);

    table.with(Style::rounded());

    if use_colors {
        table
            .with(
                Modify::new(Rows::first())
                    .with(Color::BG_BLUE)
                    .with(Color::FG_WHITE)
                    .with(Alignment::center()),
            )
            .with(
                Modify::new(Columns::new(0..1))
                    .with(Color::FG_CYAN)
                    .with(Width::wrap(20)),
            )
            .with(
                Modify::new(Columns::new(1..2))
                    .with(Color::FG_GREEN)
                    .with(Width::wrap(40)),
            )
            .with(
                Modify::new(Columns::new(2..3))
                    .with(Color::FG_YELLOW)
                    .with(Width::wrap(30)),
            )
            .with(
                Modify::new(Columns::new(3..4))
                    .with(Color::FG_MAGENTA)
                    .with(Width::wrap(15)),
            );
    } else {
        table
            .with(Modify::new(Rows::first()).with(Alignment::center()))
            .with(Modify::new(Columns::new(0..1)).with(Width::wrap(20)))
            .with(Modify::new(Columns::new(1..2)).with(Width::wrap(40)))
            .with(Modify::new(Columns::new(2..3)).with(Width::wrap(30)))
            .with(Modify::new(Columns::new(3..4)).with(Width::wrap(15)));
    }

    println!("\n{}", table);

    Ok(())
}

pub fn display_packages_table(packages: Vec<PackageEntry>, use_colors: bool) -> Result<()> {
    let mut table = Table::new(&packages);

    table.with(Style::rounded());

    if use_colors {
        table
            .with(
                Modify::new(Rows::first())
                    .with(Color::BG_BLUE)
                    .with(Color::FG_WHITE)
                    .with(Alignment::center()),
            )
            .with(
                Modify::new(Columns::new(0..1))
                    .with(Color::FG_CYAN)
                    .with(Width::wrap(25)),
            )
            .with(
                Modify::new(Columns::new(1..2))
                    .with(Color::FG_GREEN)
                    .with(Width::wrap(15)),
            )
            .with(
                Modify::new(Columns::new(2..3))
                    .with(Color::FG_YELLOW)
                    .with(Width::wrap(40)),
            )
            .with(
                Modify::new(Columns::new(3..4))
                    .with(Color::FG_MAGENTA)
                    .with(Width::wrap(10)),
            );
    } else {
        table
            .with(Modify::new(Rows::first()).with(Alignment::center()))
            .with(Modify::new(Columns::new(0..1)).with(Width::wrap(25)))
            .with(Modify::new(Columns::new(1..2)).with(Width::wrap(15)))
            .with(Modify::new(Columns::new(2..3)).with(Width::wrap(40)))
            .with(Modify::new(Columns::new(3..4)).with(Width::wrap(10)));
    }

    println!("\n{}", table);

    Ok(())
}

pub fn display_cleaned_table(entries: Vec<CleanedEntry>, use_colors: bool) -> Result<()> {
    let mut table = Table::new(&entries);

    table.with(Style::rounded());

    if use_colors {
        table
            .with(
                Modify::new(Rows::first())
                    .with(Color::BG_BLUE)
                    .with(Color::FG_WHITE)
                    .with(Alignment::center()),
            )
            .with(
                Modify::new(Columns::new(0..1))
                    .with(Color::FG_CYAN)
                    .with(Width::wrap(60)),
            )
            .with(
                Modify::new(Columns::new(1..2))
                    .with(Color::FG_YELLOW)
                    .with(Width::wrap(15)),
            )
            .with(
                Modify::new(Columns::new(2..3))
                    .with(Color::FG_GREEN)
                    .with(Width::wrap(20)),
            );
    } else {
        table
            .with(Modify::new(Rows::first()).with(Alignment::center()))
            .with(Modify::new(Columns::new(0..1)).with(Width::wrap(60)))
            .with(Modify::new(Columns::new(1..2)).with(Width::wrap(15)))
            .with(Modify::new(Columns::new(2..3)).with(Width::wrap(20)));
    }

    println!("\n{}", table);

    Ok(())
}

pub fn display_bookmarks_table(entries: Vec<BookmarkTableEntry>, use_colors: bool) -> Result<()> {
    let mut table = Table::new(&entries);

    table.with(Style::rounded());

    if use_colors {
        table
            .with(
                Modify::new(Rows::first())
                    .with(Color::BG_BLUE)
                    .with(Color::FG_WHITE)
                    .with(Alignment::center()),
            )
            .with(
                Modify::new(Columns::new(0..1))
                    .with(Color::FG_CYAN)
                    .with(Width::wrap(40)),
            )
            .with(
                Modify::new(Columns::new(1..2))
                    .with(Color::FG_GREEN)
                    .with(Width::wrap(50)),
            )
            .with(
                Modify::new(Columns::new(2..3))
                    .with(Color::FG_YELLOW)
                    .with(Width::wrap(20)),
            )
            .with(
                Modify::new(Columns::new(3..4))
                    .with(Color::FG_MAGENTA)
                    .with(Width::wrap(30)),
            );
    } else {
        table
            .with(Modify::new(Rows::first()).with(Alignment::center()))
            .with(Modify::new(Columns::new(0..1)).with(Width::wrap(40)))
            .with(Modify::new(Columns::new(1..2)).with(Width::wrap(50)))
            .with(Modify::new(Columns::new(2..3)).with(Width::wrap(20)))
            .with(Modify::new(Columns::new(3..4)).with(Width::wrap(30)));
    }

    println!("\n{}", table);

    Ok(())
}

pub fn display_duplicates_table(entries: Vec<DuplicateEntry>, use_colors: bool) -> Result<()> {
    let mut table = Table::new(&entries);

    table.with(Style::rounded());

    if use_colors {
        table
            .with(
                Modify::new(Rows::first())
                    .with(Color::BG_BLUE)
                    .with(Color::FG_WHITE)
                    .with(Alignment::center()),
            )
            .with(
                Modify::new(Columns::new(0..1))
                    .with(Color::FG_CYAN)
                    .with(Width::wrap(60)),
            )
            .with(
                Modify::new(Columns::new(1..2))
                    .with(Color::FG_YELLOW)
                    .with(Width::wrap(12)),
            )
            .with(
                Modify::new(Columns::new(2..3))
                    .with(Color::FG_GREEN)
                    .with(Width::wrap(50)),
            );
    } else {
        table
            .with(Modify::new(Rows::first()).with(Alignment::center()))
            .with(Modify::new(Columns::new(0..1)).with(Width::wrap(60)))
            .with(Modify::new(Columns::new(1..2)).with(Width::wrap(12)))
            .with(Modify::new(Columns::new(2..3)).with(Width::wrap(50)));
    }

    println!("\n{}", table);

    Ok(())
}

pub fn display_domain_stats_table(entries: Vec<DomainEntry>, use_colors: bool) -> Result<()> {
    let mut table = Table::new(&entries);

    table.with(Style::rounded());

    if use_colors {
        table
            .with(
                Modify::new(Rows::first())
                    .with(Color::BG_BLUE)
                    .with(Color::FG_WHITE)
                    .with(Alignment::center()),
            )
            .with(
                Modify::new(Columns::new(0..1))
                    .with(Color::FG_CYAN)
                    .with(Width::wrap(40)),
            )
            .with(
                Modify::new(Columns::new(1..2))
                    .with(Color::FG_YELLOW)
                    .with(Width::wrap(10)),
            )
            .with(
                Modify::new(Columns::new(2..3))
                    .with(Color::FG_GREEN)
                    .with(Width::wrap(12)),
            );
    } else {
        table
            .with(Modify::new(Rows::first()).with(Alignment::center()))
            .with(Modify::new(Columns::new(0..1)).with(Width::wrap(40)))
            .with(Modify::new(Columns::new(1..2)).with(Width::wrap(10)))
            .with(Modify::new(Columns::new(2..3)).with(Width::wrap(12)));
    }

    println!("\n{}", table);

    Ok(())
}

pub fn display_category_stats_table(entries: Vec<CategoryEntry>, use_colors: bool) -> Result<()> {
    let mut table = Table::new(&entries);

    table.with(Style::rounded());

    if use_colors {
        table
            .with(
                Modify::new(Rows::first())
                    .with(Color::BG_BLUE)
                    .with(Color::FG_WHITE)
                    .with(Alignment::center()),
            )
            .with(
                Modify::new(Columns::new(0..1))
                    .with(Color::FG_CYAN)
                    .with(Width::wrap(25)),
            )
            .with(
                Modify::new(Columns::new(1..2))
                    .with(Color::FG_YELLOW)
                    .with(Width::wrap(10)),
            )
            .with(
                Modify::new(Columns::new(2..3))
                    .with(Color::FG_GREEN)
                    .with(Width::wrap(12)),
            );
    } else {
        table
            .with(Modify::new(Rows::first()).with(Alignment::center()))
            .with(Modify::new(Columns::new(0..1)).with(Width::wrap(25)))
            .with(Modify::new(Columns::new(1..2)).with(Width::wrap(10)))
            .with(Modify::new(Columns::new(2..3)).with(Width::wrap(12)));
    }

    println!("\n{}", table);

    Ok(())
}

pub fn display_organize_suggestions_table(
    entries: Vec<OrganizeSuggestion>,
    use_colors: bool,
) -> Result<()> {
    let mut table = Table::new(&entries);

    table.with(Style::rounded());

    if use_colors {
        table
            .with(
                Modify::new(Rows::first())
                    .with(Color::BG_BLUE)
                    .with(Color::FG_WHITE)
                    .with(Alignment::center()),
            )
            .with(
                Modify::new(Columns::new(0..1))
                    .with(Color::FG_CYAN)
                    .with(Width::wrap(40)),
            )
            .with(
                Modify::new(Columns::new(1..2))
                    .with(Color::FG_RED)
                    .with(Width::wrap(30)),
            )
            .with(
                Modify::new(Columns::new(2..3))
                    .with(Color::FG_GREEN)
                    .with(Width::wrap(20)),
            )
            .with(
                Modify::new(Columns::new(3..4))
                    .with(Color::FG_YELLOW)
                    .with(Width::wrap(20)),
            );
    } else {
        table
            .with(Modify::new(Rows::first()).with(Alignment::center()))
            .with(Modify::new(Columns::new(0..1)).with(Width::wrap(40)))
            .with(Modify::new(Columns::new(1..2)).with(Width::wrap(30)))
            .with(Modify::new(Columns::new(2..3)).with(Width::wrap(20)))
            .with(Modify::new(Columns::new(3..4)).with(Width::wrap(20)));
    }

    println!("\n{}", table);

    Ok(())
}
