use anyhow::Result;
use tabled::{
    settings::{
        object::{Columns, Rows},
        Alignment, Color, Modify, Style, Width,
    },
    Table,
};

use crate::{AliasEntry, FunctionEntry, PackageEntry};

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
            .with(
                Modify::new(Rows::first())
                    .with(Alignment::center()),
            )
            .with(
                Modify::new(Columns::new(0..1))
                    .with(Width::wrap(20)),
            )
            .with(
                Modify::new(Columns::new(1..2))
                    .with(Width::wrap(50)),
            )
            .with(
                Modify::new(Columns::new(2..3))
                    .with(Width::wrap(15)),
            );
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
            .with(
                Modify::new(Rows::first())
                    .with(Alignment::center()),
            )
            .with(
                Modify::new(Columns::new(0..1))
                    .with(Width::wrap(20)),
            )
            .with(
                Modify::new(Columns::new(1..2))
                    .with(Width::wrap(40)),
            )
            .with(
                Modify::new(Columns::new(2..3))
                    .with(Width::wrap(30)),
            )
            .with(
                Modify::new(Columns::new(3..4))
                    .with(Width::wrap(15)),
            );
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
            .with(
                Modify::new(Rows::first())
                    .with(Alignment::center()),
            )
            .with(
                Modify::new(Columns::new(0..1))
                    .with(Width::wrap(25)),
            )
            .with(
                Modify::new(Columns::new(1..2))
                    .with(Width::wrap(15)),
            )
            .with(
                Modify::new(Columns::new(2..3))
                    .with(Width::wrap(40)),
            )
            .with(
                Modify::new(Columns::new(3..4))
                    .with(Width::wrap(10)),
            );
    }
    
    println!("\n{}", table);
    
    Ok(())
}
