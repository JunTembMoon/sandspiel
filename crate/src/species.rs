use super::utils::*;
use Cell;
use SandApi;
use Wind;
use EMPTY_CELL;

// use std::cmp;
use std::mem;
use wasm_bindgen::prelude::*;
// use web_sys::console;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Species {
    Empty = 0,
    Wall = 1,
    Sand = 2,
    Water = 3,
    // X = 21,
    Stone = 13,
    Ice = 9,
    Gas = 4,
    Cloner = 5,
    // Sink = 10,
    Mite = 15,
    Wood = 7,
    Plant = 11,
    Fungus = 18,
    Seed = 19,
    Fire = 6,
    Lava = 8,
    Acid = 12,
    Dust = 14,
    Oil = 16,
    Rocket = 17,
    Glass = 20,
    MoltenGlass = 21,
    Steam = 22,
    Slime = 23,
    Metal = 24,
    Electricity = 25,
    Nitro = 26,
    WetSand = 27,
    SaltWater = 28,
    Salt = 29,
}

impl Species {
    pub fn update(&self, cell: Cell, api: SandApi) {
        match self {
            Species::Empty => {}
            Species::Wall => {}
            Species::Sand => update_sand(cell, api),
            Species::WetSand => update_wet_sand(cell, api),
            Species::SaltWater => update_salt_water(cell, api),
            Species::Salt => update_salt(cell, api),
            Species::Dust => update_dust(cell, api),
            Species::Water => update_water(cell, api),
            Species::Stone => update_stone(cell, api),
            Species::Gas => update_gas(cell, api),
            Species::Cloner => update_cloner(cell, api),
            Species::Rocket => update_rocket(cell, api),
            Species::Fire => update_fire(cell, api),
            Species::Wood => update_wood(cell, api),
            Species::Lava => update_lava(cell, api),
            Species::Ice => update_ice(cell, api),
            // Species::Snow => update_ice(cell, api),
            //lightning
            // Species::Sink => update_sink(cell, api),
            Species::Plant => update_plant(cell, api),
            Species::Acid => update_acid(cell, api),
            Species::Mite => update_mite(cell, api),
            Species::Oil => update_oil(cell, api),
            Species::Fungus => update_fungus(cell, api),
            Species::Seed => update_seed(cell, api),
            Species::Glass => update_glass(cell, api),
            Species::MoltenGlass => update_molten_glass(cell, api),
            Species::Steam => update_steam(cell, api),
            Species::Slime => update_slime(cell, api),
            Species::Metal => update_metal(cell, api),
            Species::Electricity => update_electricity(cell, api),
            Species::Nitro => update_nitro(cell, api),
            // Species::X => update_x(cell, api),
        }
    }
}

pub fn update_electricity(cell: Cell, mut api: SandApi) {
    let mut next = cell;
    next.ra = cell.ra.saturating_sub(20);
    if next.ra < 20 {
        api.set(0, 0, EMPTY_CELL);
        return;
    }

    let (dx, dy) = api.rand_vec_8();
    let target = api.get(dx, dy).species;
    if target == Species::Nitro
        || target == Species::Oil
        || target == Species::Gas
        || target == Species::Wood
        || target == Species::Plant
        || target == Species::Slime
    {
        api.set(
            dx,
            dy,
            Cell {
                species: Species::Fire,
                ra: 130,
                rb: 0,
                clock: 0,
            },
        );
    } else if target == Species::Water {
        api.set(
            dx,
            dy,
            Cell {
                species: Species::Steam,
                ra: 100,
                rb: 4,
                clock: 0,
            },
        );
    }

    if target == Species::Empty || target == Species::Metal || target == Species::Steam {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, dy, next);
    } else {
        api.set(0, 0, next);
    }
}

pub fn update_nitro(cell: Cell, mut api: SandApi) {
    let mut armed = cell.rb;
    for sx in -1..=1 {
        for sy in -1..=1 {
            if sx == 0 && sy == 0 {
                continue;
            }
            let n = api.get(sx, sy).species;
            if n == Species::Fire || n == Species::Lava || n == Species::Electricity {
                armed = 8;
            }
        }
    }

    if armed > 0 {
        for sx in -1..=1 {
            for sy in -1..=1 {
                let fire_ra = 110 + api.rand_int(60) as u8;
                api.set(
                    sx,
                    sy,
                    Cell {
                        species: Species::Fire,
                        ra: fire_ra,
                        rb: 0,
                        clock: 0,
                    },
                );
            }
        }
        api.set_fluid(Wind {
            dx: 126,
            dy: 126,
            pressure: 180,
            density: 30,
        });
        return;
    }

    let dx = api.rand_dir_2();
    let below = api.get(0, 1);
    if below.species == Species::Empty || below.species == Species::Oil {
        api.set(0, 0, below);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if api.get(dx, 0).species == Species::Empty && api.once_in(5) {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else {
        api.set(0, 0, Cell { rb: 0, ..cell });
    }
}

pub fn update_steam(cell: Cell, mut api: SandApi) {
    let hot_neighbor = api.get(0, 1).species == Species::Fire
        || api.get(0, 1).species == Species::Lava
        || api.get(0, 1).species == Species::MoltenGlass
        || api.get(0, -1).species == Species::Fire
        || api.get(0, -1).species == Species::Lava
        || api.get(0, -1).species == Species::MoltenGlass;

    let mut next = cell;
    next.ra = if hot_neighbor {
        140 + api.rand_int(50) as u8
    } else if api.once_in(4) {
        cell.ra.saturating_sub(1)
    } else {
        cell.ra
    };

    if next.ra < 35 {
        api.set(
            0,
            0,
            Cell {
                species: Species::Water,
                ra: 110,
                rb: 0,
                clock: 0,
            },
        );
        return;
    }

    let dx = api.rand_dir_2();
    if api.get(0, -1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, -1, next);
    } else if api.get(dx, -1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, -1, next);
    } else if api.get(dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, next);
    } else {
        api.set(0, 0, next);
    }
}

pub fn update_slime(cell: Cell, mut api: SandApi) {
    let dx = api.rand_dir_2();
    let below = api.get(0, 1);

    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, Cell { rb: 6, ..cell });
        return;
    }

    if cell.rb == 0 && api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, Cell { rb: 7, ..cell });
        return;
    }

    if cell.rb == 0 && api.once_in(7) && api.get(dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, Cell { rb: 8, ..cell });
        return;
    }

    let hot = api.get(0, -1).species == Species::Fire
        || api.get(0, -1).species == Species::Lava
        || api.get(0, 1).species == Species::Fire
        || api.get(0, 1).species == Species::Lava;
    if hot && api.once_in(6) {
        let gas_ra = 80 + api.rand_int(40) as u8;
        api.set(
            0,
            0,
            Cell {
                species: Species::Gas,
                ra: gas_ra,
                rb: 3,
                clock: 0,
            },
        );
        return;
    }

    api.set(
        0,
        0,
        Cell {
            rb: cell.rb.saturating_sub(1),
            ..cell
        },
    );
}

pub fn update_metal(cell: Cell, mut api: SandApi) {
    let mut heated = false;
    for sx in -1..=1 {
        for sy in -1..=1 {
            if sx == 0 && sy == 0 {
                continue;
            }
            let n = api.get(sx, sy).species;
            if n == Species::Fire || n == Species::Lava || n == Species::MoltenGlass {
                heated = true;
                break;
            }
        }
        if heated {
            break;
        }
    }

    let mut next = cell;
    next.rb = if heated {
        (cell.rb + 8).min(180)
    } else {
        cell.rb.saturating_sub(1)
    };
    next.ra = 60 + next.rb;

    if next.rb > 60 {
        for &(sx, sy) in &[(0, -1), (0, 1), (-1, 0), (1, 0)] {
            if api.get(sx, sy).species == Species::Water {
                let steam_ra = 90 + api.rand_int(40) as u8;
                api.set(
                    sx,
                    sy,
                    Cell {
                        species: Species::Steam,
                        ra: steam_ra,
                        rb: 5,
                        clock: 0,
                    },
                );
                next.rb = next.rb.saturating_sub(20);
                next.ra = 60 + next.rb;
                break;
            }
        }
    }

    api.set(0, 0, next);
}

pub fn update_glass(cell: Cell, mut api: SandApi) {
    let mut heated = false;
    for sx in -1..=1 {
        for sy in -1..=1 {
            if sx == 0 && sy == 0 {
                continue;
            }
            let neighbor = api.get(sx, sy).species;
            if neighbor == Species::Fire || neighbor == Species::Lava {
                heated = true;
                break;
            }
        }
        if heated {
            break;
        }
    }

    if heated && api.once_in(4) {
        api.set(
            0,
            0,
            Cell {
                species: Species::MoltenGlass,
                ra: 160,
                rb: 3,
                clock: 0,
            },
        );
    }
}

pub fn update_molten_glass(cell: Cell, mut api: SandApi) {
    let mut heated = false;
    for sx in -1..=1 {
        for sy in -1..=1 {
            if sx == 0 && sy == 0 {
                continue;
            }
            let neighbor = api.get(sx, sy).species;
            if neighbor == Species::Fire || neighbor == Species::Lava {
                heated = true;
                break;
            }
        }
        if heated {
            break;
        }
    }

    // Water cools molten glass and a small amount of water flashes off.
    let water_below = api.get(0, 1).species == Species::Water;
    let water_above = api.get(0, -1).species == Species::Water;
    let water_left = api.get(-1, 0).species == Species::Water;
    let water_right = api.get(1, 0).species == Species::Water;
    if water_below || water_above || water_left || water_right {
        let mut smoke_dx = 0;
        let mut smoke_dy = -1;

        if water_below {
            api.set(0, 1, EMPTY_CELL);
            smoke_dx = 0;
            smoke_dy = 1;
        } else if water_above {
            api.set(0, -1, EMPTY_CELL);
            smoke_dx = 0;
            smoke_dy = -1;
        } else if water_left {
            api.set(-1, 0, EMPTY_CELL);
            smoke_dx = -1;
            smoke_dy = 0;
        } else if water_right {
            api.set(1, 0, EMPTY_CELL);
            smoke_dx = 1;
            smoke_dy = 0;
        }

        let steam_ra = 35 + api.rand_int(30) as u8;
        if api.get(smoke_dx, smoke_dy).species == Species::Empty {
            api.set(
                smoke_dx,
                smoke_dy,
                Cell {
                    species: Species::Gas,
                    ra: steam_ra,
                    rb: 4,
                    clock: 0,
                },
            );
        } else if api.get(0, -1).species == Species::Empty {
            api.set(
                0,
                -1,
                Cell {
                    species: Species::Gas,
                    ra: steam_ra,
                    rb: 4,
                    clock: 0,
                },
            );
        }

        api.set_fluid(Wind {
            dx: 0,
            dy: 24,
            pressure: 28,
            density: 16,
        });

        api.set(
            0,
            0,
            Cell {
                species: Species::Glass,
                ra: 120,
                rb: 0,
                clock: 0,
            },
        );
        return;
    }

    let mut cooled = cell;
    cooled.ra = if heated {
        165 + api.rand_int(55) as u8
    } else if api.once_in(8) {
        cell.ra.saturating_sub(1)
    } else {
        cell.ra
    };

    // Very hot molten glass can ignite nearby flammables.
    if cooled.ra > 120 && api.once_in(5) {
        let (ix, iy) = api.rand_vec_8();
        let target = api.get(ix, iy).species;
        if target == Species::Gas
            || target == Species::Oil
            || target == Species::Wood
            || target == Species::Plant
            || target == Species::Fungus
            || target == Species::Seed
            || target == Species::Dust
        {
            let fire_ra = 80 + api.rand_int(70) as u8;
            api.set(
                ix,
                iy,
                Cell {
                    species: Species::Fire,
                    ra: fire_ra,
                    rb: 0,
                    clock: 0,
                },
            );
        }
    }

    if !heated && cooled.ra < 25 {
        let smoke_ra = 25 + api.rand_int(20) as u8;
        if api.get(0, -1).species == Species::Empty {
            api.set(
                0,
                -1,
                Cell {
                    species: Species::Gas,
                    ra: smoke_ra,
                    rb: 3,
                    clock: 0,
                },
            );
        } else if api.get(-1, -1).species == Species::Empty {
            api.set(
                -1,
                -1,
                Cell {
                    species: Species::Gas,
                    ra: smoke_ra,
                    rb: 3,
                    clock: 0,
                },
            );
        } else if api.get(1, -1).species == Species::Empty {
            api.set(
                1,
                -1,
                Cell {
                    species: Species::Gas,
                    ra: smoke_ra,
                    rb: 3,
                    clock: 0,
                },
            );
        }

        api.set(
            0,
            0,
            Cell {
                species: Species::Glass,
                ra: 110,
                rb: 0,
                clock: 0,
            },
        );
        return;
    }

    let dx = api.rand_dir_2();
    let rb = cell.rb;
    let above_species = api.get(0, -1).species;
    let anchored_column = above_species == Species::MoltenGlass || above_species == Species::Glass;

    if rb > 0 {
        api.set(
            0,
            0,
            Cell {
                rb: rb.saturating_sub(1),
                ..cooled
            },
        );
        return;
    }

    let can_fall = api.get(0, 1).species == Species::Empty;
    if can_fall {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, Cell { rb: 10, ..cooled });
        return;
    }

    let can_slide_diag = api.get(dx, 1).species == Species::Empty;
    if can_slide_diag && !anchored_column && api.once_in(10) {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, Cell { rb: 14, ..cooled });
        return;
    }

    let can_slide_side = api.get(dx, 0).species == Species::Empty;
    if can_slide_side && !anchored_column && api.once_in(20) {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, Cell { rb: 18, ..cooled });
        return;
    }

    api.set(
        0,
        0,
        Cell {
            rb: 4,
            ..cooled
        },
    );
}

pub fn update_sand(cell: Cell, mut api: SandApi) {
    let mut heated = false;
    for sx in -1..=1 {
        for sy in -1..=1 {
            if sx == 0 && sy == 0 {
                continue;
            }
            let neighbor = api.get(sx, sy).species;
            if neighbor == Species::Fire || neighbor == Species::Lava {
                heated = true;
                break;
            }
        }
        if heated {
            break;
        }
    }

    if heated && api.once_in(4) {
        let glass_ra = 120 + api.rand_int(70) as u8;
        api.set(
            0,
            0,
            Cell {
                species: Species::Glass,
                ra: glass_ra,
                rb: 0,
                clock: 0,
            },
        );
        return;
    }

    for &(sx, sy) in &[(0, 1), (0, -1), (-1, 0), (1, 0)] {
        let neighbor = api.get(sx, sy);
        if neighbor.species == Species::Water {
            api.set(sx, sy, EMPTY_CELL);
            api.set(
                0,
                0,
                Cell {
                    species: Species::WetSand,
                    ra: cell.ra.saturating_sub(24),
                    rb: 4,
                    clock: 0,
                },
            );
            return;
        }
        if neighbor.species == Species::SaltWater && api.once_in(6) {
            api.set(
                0,
                0,
                Cell {
                    species: Species::WetSand,
                    ra: cell.ra.saturating_sub(10),
                    rb: 1,
                    clock: 0,
                },
            );
            return;
        }
    }

    let dx = api.rand_dir_2();

    let nbr = api.get(0, 1);
    if nbr.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if nbr.species == Species::Water
        || nbr.species == Species::Gas
        || nbr.species == Species::Oil
        || nbr.species == Species::Acid
    {
        api.set(0, 0, nbr);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

pub fn update_wet_sand(cell: Cell, mut api: SandApi) {
    let mut heated = false;
    for sx in -1..=1 {
        for sy in -1..=1 {
            if sx == 0 && sy == 0 {
                continue;
            }
            let neighbor = api.get(sx, sy).species;
            if neighbor == Species::Fire || neighbor == Species::Lava {
                heated = true;
                break;
            }
        }
        if heated {
            break;
        }
    }

    if heated && api.once_in(4) {
        let glass_ra = 120 + api.rand_int(70) as u8;
        api.set(
            0,
            0,
            Cell {
                species: Species::Glass,
                ra: glass_ra,
                rb: 0,
                clock: 0,
            },
        );
        return;
    }

    for &(sx, sy) in &[(0, 1), (0, -1), (-1, 0), (1, 0)] {
        let neighbor = api.get(sx, sy).species;
        if neighbor == Species::Water {
            api.set(
                0,
                0,
                Cell {
                    rb: (cell.rb + 1).min(10),
                    ra: cell.ra.saturating_sub(1),
                    ..cell
                },
            );
            return;
        }
        if neighbor == Species::SaltWater {
            api.set(
                0,
                0,
                Cell {
                    rb: (cell.rb + 1).min(8),
                    ra: cell.ra.saturating_sub(1),
                    ..cell
                },
            );
            return;
        }
    }

    let dx = api.rand_dir_2();
    let below = api.get(0, 1);
    if below.species == Species::Sand {
        api.set(
            0,
            1,
            Cell {
                species: Species::WetSand,
                ra: below.ra.saturating_sub(22),
                rb: (cell.rb + 2).min(10),
                clock: 0,
            },
        );
        api.set(
            0,
            0,
            Cell {
                rb: cell.rb.saturating_sub(1),
                ..cell
            },
        );
        return;
    }
    if below.species == Species::WetSand && below.rb < 9 && cell.rb > 0 && api.once_in(2) {
        api.set(
            0,
            1,
            Cell {
                rb: (below.rb + 1).min(10),
                ..below
            },
        );
        api.set(
            0,
            0,
            Cell {
                rb: cell.rb.saturating_sub(1),
                ..cell
            },
        );
        return;
    }

    let fluid_below = below.species == Species::Water
        || below.species == Species::SaltWater
        || below.species == Species::Gas
        || below.species == Species::Oil
        || below.species == Species::Acid;

    if below.species == Species::Empty || fluid_below {
        api.set(0, 0, if fluid_below { below } else { EMPTY_CELL });
        api.set(
            0,
            1,
            Cell {
                rb: (cell.rb + 2).min(10),
                ..cell
            },
        );
        return;
    }

    if cell.rb >= 6 {
        let crack_left = api.get(-1, 1).species == Species::Empty;
        let crack_right = api.get(1, 1).species == Species::Empty;
        let cracked = Cell {
            species: Species::Sand,
            ra: cell.ra.saturating_sub(8),
            rb: 0,
            clock: 0,
        };

        if crack_left || crack_right {
            let crack_dx = if crack_left && crack_right {
                dx
            } else if crack_left {
                -1
            } else {
                1
            };
            api.set(0, 0, EMPTY_CELL);
            api.set(crack_dx, 1, cracked);
        } else {
            api.set(0, 0, cracked);
        }
        return;
    }

    let diag = api.get(dx, 1);
    if cell.rb <= 1
        && (diag.species == Species::Empty
            || diag.species == Species::Water
            || diag.species == Species::SaltWater
            || diag.species == Species::Oil)
    {
        api.set(0, 0, if diag.species == Species::Empty { EMPTY_CELL } else { diag });
        api.set(
            dx,
            1,
            Cell {
                rb: (cell.rb + 1).min(10),
                ..cell
            },
        );
        return;
    }

    if cell.rb == 0 && api.once_in(900) {
        api.set(
            0,
            0,
            Cell {
                species: Species::Sand,
                ra: cell.ra.saturating_add(6),
                rb: 0,
                clock: 0,
            },
        );
        return;
    }

    let should_dry = api.once_in(10);
    api.set(
        0,
        0,
        Cell {
            rb: if should_dry {
                cell.rb.saturating_sub(1)
            } else {
                cell.rb
            },
            ..cell
        },
    );
}

pub fn update_dust(cell: Cell, mut api: SandApi) {
    let dx = api.rand_dir();
    let fluid = api.get_fluid();

    if fluid.pressure > 120 {
        api.set(
            0,
            0,
            Cell {
                species: Species::Fire,
                ra: (150 + (cell.ra / 10)) as u8,
                rb: 0,
                clock: 0,
            },
        );
        api.set_fluid(Wind {
            dx: 0,
            dy: 0,
            pressure: 80,
            density: 5,
        });
        return;
    }

    let nbr = api.get(0, 1);
    if nbr.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if nbr.species == Species::Water {
        api.set(0, 0, nbr);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

pub fn update_stone(cell: Cell, mut api: SandApi) {
    if api.get(-1, -1).species == Species::Stone && api.get(1, -1).species == Species::Stone {
        return;
    }
    let fluid = api.get_fluid();

    if fluid.pressure > 120 && api.rand_int(1) == 0 {
        api.set(
            0,
            0,
            Cell {
                species: Species::Sand,
                ra: cell.ra,
                rb: 0,
                clock: 0,
            },
        );
        return;
    }

    let nbr = api.get(0, 1);
    let nbr_species = nbr.species;
    if nbr_species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if nbr_species == Species::Water
        || nbr_species == Species::Gas
        || nbr_species == Species::Oil
        || nbr_species == Species::Acid
    {
        api.set(0, 0, nbr);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

pub fn update_water(cell: Cell, mut api: SandApi) {
    let mut boiling = false;
    for sx in -1..=1 {
        for sy in -1..=1 {
            if sx == 0 && sy == 0 {
                continue;
            }
            let n = api.get(sx, sy).species;
            if n == Species::Fire || n == Species::Lava || n == Species::MoltenGlass || n == Species::Metal {
                boiling = true;
                break;
            }
        }
        if boiling {
            break;
        }
    }

    if boiling && api.once_in(8) {
        let steam_ra = 120 + api.rand_int(40) as u8;
        api.set(
            0,
            0,
            Cell {
                species: Species::Steam,
                ra: steam_ra,
                rb: 5,
                clock: 0,
            },
        );
        return;
    }

    let below = api.get(0, 1);
    if below.species == Species::Sand {
        api.set(
            0,
            1,
            Cell {
                species: Species::WetSand,
                ra: below.ra.saturating_sub(24),
                rb: 6,
                clock: 0,
            },
        );
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    if below.species == Species::WetSand && below.rb < 9 {
        api.set(
            0,
            1,
            Cell {
                rb: (below.rb + 2).min(10),
                ..below
            },
        );
        api.set(0, 0, EMPTY_CELL);
        return;
    }

    for &(sx, sy) in &[(0, 1), (-1, 1), (1, 1), (-1, 0), (1, 0)] {
        let neighbor = api.get(sx, sy);
        if neighbor.species == Species::Sand {
            api.set(
                sx,
                sy,
                Cell {
                    species: Species::WetSand,
                    ra: neighbor.ra.saturating_sub(20),
                    rb: 4,
                    clock: 0,
                },
            );
            if api.once_in(3) {
                api.set(0, 0, EMPTY_CELL);
            }
            return;
        }
    }

    let mut dx = api.rand_dir();
    let below = api.get(0, 1);
    let dx1 = api.get(dx, 1);
    // let mut dx0 = api.get(dx, 0);
    //fall down
    if below.species == Species::Empty || below.species == Species::Oil {
        api.set(0, 0, below);
        let mut ra = cell.ra;
        if api.once_in(20) {
            //randomize direction when falling sometimes
            ra = 100 + api.rand_int(50) as u8;
        }
        api.set(0, 1, Cell { ra, ..cell });

        return;
    } else if dx1.species == Species::Empty || dx1.species == Species::Oil {
        //fall diagonally
        api.set(0, 0, dx1);
        api.set(dx, 1, cell);
        return;
    } else if api.get(-dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 1, cell);
        return;
    }
    let left = cell.ra % 2 == 0;
    dx = if left { 1 } else { -1 };
    let dx0 = api.get(dx, 0);
    let dxd = api.get(dx * 2, 0);

    if dx0.species == Species::Empty && dxd.species == Species::Empty {
        // scoot double
        api.set(0, 0, dxd);
        api.set(2 * dx, 0, Cell { rb: 6, ..cell });
        let (dx, dy) = api.rand_vec_8();
        let nbr = api.get(dx, dy);

        // spread opinion
        if nbr.species == Species::Water {
            if nbr.ra % 2 != cell.ra % 2 {
                api.set(
                    dx,
                    dy,
                    Cell {
                        ra: cell.ra,
                        ..cell
                    },
                )
            }
        }
    } else if dx0.species == Species::Empty || dx0.species == Species::Oil {
        api.set(0, 0, dx0);
        api.set(dx, 0, Cell { rb: 3, ..cell });
        let (dx, dy) = api.rand_vec_8();
        let nbr = api.get(dx, dy);
        if nbr.species == Species::Water {
            if nbr.ra % 2 != cell.ra % 2 {
                api.set(
                    dx,
                    dy,
                    Cell {
                        ra: cell.ra,
                        ..cell
                    },
                )
            }
        }
    } else if cell.rb == 0 {
        if api.get(-dx, 0).species == Species::Empty {
            // bump
            api.set(
                0,
                0,
                Cell {
                    ra: ((cell.ra as i32) + dx) as u8,
                    ..cell
                },
            );
        }
    } else {
        // become less certain (more bumpable)
        api.set(
            0,
            0,
            Cell {
                rb: cell.rb - 1,
                ..cell
            },
        );
    }
    // if api.once_in(8) {
    //     let (dx, dy) = api.rand_vec_8();
    //     let nbr = api.get(dx, dy);
    //     if nbr.species == Species::Water {
    //         if nbr.ra % 2 != cell.ra % 2 {
    //             api.set(0, 0, Cell { ra: nbr.ra, ..cell })
    //         }
    //     }
    // }

    // let (dx, dy) = api.rand_vec_8();
    // let nbr = api.get(dx, dy);
    // if nbr.species == Species::Water {
    //     if nbr.ra % 2 != cell.ra % 2 && api.once_in(2) {
    //         api.set(0, 0, Cell { ra: nbr.ra, ..cell })
    //     }
    // }

    // {

    // if api.get(-dx, 0).species == Species::Empty {
    //     api.set(0, 0, EMPTY_CELL);
    //     api.set(-dx, 0, cell);
    // }
}

pub fn update_salt_water(cell: Cell, mut api: SandApi) {
    let mut burning = false;
    for sx in -1..=1 {
        for sy in -1..=1 {
            if sx == 0 && sy == 0 {
                continue;
            }
            let n = api.get(sx, sy).species;
            if n == Species::Fire || n == Species::Lava || n == Species::MoltenGlass {
                burning = true;
                break;
            }
        }
        if burning {
            break;
        }
    }

    if burning && api.once_in(4) {
        if api.get(0, -1).species == Species::Empty {
            let steam_ra = 100 + api.rand_int(30) as u8;
            api.set(
                0,
                -1,
                Cell {
                    species: Species::Steam,
                    ra: steam_ra,
                    rb: 4,
                    clock: 0,
                },
            );
        }
        api.set(
            0,
            0,
            Cell {
                species: Species::Salt,
                ra: 150,
                rb: 0,
                clock: 0,
            },
        );
        return;
    }

    for &(sx, sy) in &[(0, 1), (-1, 1), (1, 1), (-1, 0), (1, 0)] {
        let neighbor = api.get(sx, sy);
        if neighbor.species == Species::Sand && api.once_in(14) {
            api.set(
                sx,
                sy,
                Cell {
                    species: Species::WetSand,
                    ra: neighbor.ra.saturating_sub(8),
                    rb: 1,
                    clock: 0,
                },
            );
            break;
        }
    }

    let mut dx = api.rand_dir();
    let below = api.get(0, 1);
    let dx1 = api.get(dx, 1);
    if below.species == Species::Empty || below.species == Species::Oil {
        api.set(0, 0, below);
        api.set(0, 1, Cell { rb: cell.rb.saturating_add(1), ..cell });
        return;
    } else if dx1.species == Species::Empty || dx1.species == Species::Oil {
        api.set(0, 0, dx1);
        api.set(dx, 1, cell);
        return;
    } else if api.get(-dx, 1).species == Species::Empty && api.once_in(2) {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 1, cell);
        return;
    }

    let left = cell.ra % 2 == 0;
    dx = if left { 1 } else { -1 };
    let side = api.get(dx, 0);
    if side.species == Species::Empty && api.once_in(3) {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
        return;
    }

    api.set(0, 0, Cell { rb: cell.rb.saturating_sub(1), ..cell });
}

pub fn update_salt(cell: Cell, mut api: SandApi) {
    for &(sx, sy) in &[(0, -1), (-1, 0), (1, 0)] {
        let neighbor = api.get(sx, sy);
        if neighbor.species == Species::Water {
            api.set(
                sx,
                sy,
                Cell {
                    species: Species::SaltWater,
                    ra: neighbor.ra,
                    rb: 4,
                    clock: 0,
                },
            );
            if api.once_in(2) {
                api.set(0, 0, EMPTY_CELL);
                return;
            }
        }
    }

    let dx = api.rand_dir_2();
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
        return;
    }
    if below.species == Species::Water {
        api.set(
            0,
            1,
            Cell {
                species: Species::SaltWater,
                ra: below.ra,
                rb: 4,
                clock: 0,
            },
        );
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
        return;
    }

    api.set(0, 0, cell);
}

pub fn update_oil(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;
    let (dx, dy) = api.rand_vec();

    let mut new_cell = cell;
    let nbr = api.get(dx, dy);
    if rb == 0 && nbr.species == Species::Fire
        || nbr.species == Species::Lava
        || (nbr.species == Species::Oil && nbr.rb > 1 && nbr.rb < 20)
    {
        new_cell = Cell {
            species: Species::Oil,
            ra: cell.ra,
            rb: 50,
            clock: 0,
        };
    }

    if rb > 1 {
        new_cell = Cell {
            species: Species::Oil,
            ra: cell.ra,
            rb: rb - 1,
            clock: 0,
        };
        api.set_fluid(Wind {
            dx: 0,
            dy: 10,
            pressure: 10,
            density: 180,
        });
        if rb % 4 != 0 && nbr.species == Species::Empty && nbr.species != Species::Water {
            let ra = 20 + api.rand_int(30) as u8;
            api.set(
                dx,
                dy,
                Cell {
                    species: Species::Fire,
                    ra,
                    rb: 0,
                    clock: 0,
                },
            );
        }
        if nbr.species == Species::Water {
            new_cell = Cell {
                species: Species::Oil,
                ra: 50,
                rb: 0,
                clock: 0,
            };
        }
    } else if rb == 1 {
        api.set(
            0,
            0,
            Cell {
                species: Species::Empty,
                ra: cell.ra,
                rb: 90,
                clock: 0,
            },
        );
        return;
    }

    if api.get(0, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, new_cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, new_cell);
    } else if api.get(-dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 1, new_cell);
    } else if api.get(dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, new_cell);
    } else if api.get(-dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 0, new_cell);
    } else {
        api.set(0, 0, new_cell);
    }
}

pub fn update_gas(cell: Cell, mut api: SandApi) {
    if api.universe.gas_overload {
        let ignition_ra = 120 + api.rand_int(80) as u8;
        api.set(
            0,
            0,
            Cell {
                species: Species::Fire,
                ra: ignition_ra,
                rb: 0,
                clock: 0,
            },
        );
        api.set_fluid(Wind {
            dx: 0,
            dy: 0,
            pressure: 120,
            density: 40,
        });
        return;
    }

    // Ignite when gas is packed in a sealed local chamber.
    let mut inner_gas = 0;
    for sx in -1..=1 {
        for sy in -1..=1 {
            if api.get(sx, sy).species == Species::Gas {
                inner_gas += 1;
            }
        }
    }

    let mut shell_blocked = 0;
    for sx in -2..=2 {
        for sy in -2..=2 {
            if sx == -2 || sx == 2 || sy == -2 || sy == 2 {
                let species = api.get(sx, sy).species;
                if species != Species::Empty && species != Species::Gas && species != Species::Fire {
                    shell_blocked += 1;
                }
            }
        }
    }

    if inner_gas >= 8 && shell_blocked >= 12 {
        let ignition_ra = 95 + api.rand_int(65) as u8;
        api.set(
            0,
            0,
            Cell {
                species: Species::Fire,
                ra: ignition_ra,
                rb: 0,
                clock: 0,
            },
        );
        api.set_fluid(Wind {
            dx: 0,
            dy: 0,
            pressure: 100,
            density: 35,
        });
        return;
    }

    let (dx, dy) = api.rand_vec();

    let nbr = api.get(dx, dy);
    // api.set_fluid(Wind {
    //     dx: 0,
    //     dy: 0,
    //     pressure: 5,
    //     density: 0,
    // });
    if cell.rb == 0 {
        api.set(0, 0, Cell { rb: 5, ..cell });
    }

    if nbr.species == Species::Empty {
        if cell.rb < 3 {
            //single molecule
            api.set(0, 0, EMPTY_CELL);
            api.set(dx, dy, cell);
        } else {
            api.set(0, 0, Cell { rb: 1, ..cell });
            api.set(
                dx,
                dy,
                Cell {
                    rb: cell.rb - 1,
                    ..cell
                },
            );
        }
    } else if (dx != 0 || dy != 0) && nbr.species == Species::Gas && nbr.rb < 4 {
        // if (cell.rb < 2) {
        api.set(0, 0, EMPTY_CELL);
        // }
        api.set(
            dx,
            dy,
            Cell {
                rb: nbr.rb + cell.rb,
                ..cell
            },
        );
    }
}
// pub fn update_x(cell: Cell, mut api: SandApi) {
//     let (dx, dy) = api.rand_vec_8();

//     let nbr = api.get(dx, dy);

//     if nbr.species == Species::X {
//         let opposite = api.get(-dx, -dy);
//         if opposite.species == Species::Empty {
//             api.set(0, 0, EMPTY_CELL);
//             api.set(-dx, -dy, cell);
//         }
//     }
// }

pub fn update_cloner(cell: Cell, mut api: SandApi) {
    let mut clone_species = unsafe { mem::transmute(cell.rb as u8) };
    let g = api.universe.generation;
    for dx in [-1, 0, 1].iter().cloned() {
        for dy in [-1, 0, 1].iter().cloned() {
            if cell.rb == 0 {
                let nbr_species = api.get(dx, dy).species;
                if nbr_species != Species::Empty
                    && nbr_species != Species::Cloner
                    && nbr_species != Species::Wall
                {
                    clone_species = nbr_species;
                    api.set(
                        0,
                        0,
                        Cell {
                            species: cell.species,
                            ra: 200,
                            rb: clone_species as u8,
                            clock: 0,
                        },
                    );

                    break;
                }
            } else {
                if api.rand_int(100) > 90 && api.get(dx, dy).species == Species::Empty {
                    let ra = 80 + api.rand_int(30) as u8 + ((g % 127) as i8 - 60).abs() as u8;
                    api.set(
                        dx,
                        dy,
                        Cell {
                            species: clone_species,
                            ra,
                            rb: 0,
                            clock: 0,
                        },
                    );
                    break;
                }
            }
        }
    }
}

pub fn update_rocket(cell: Cell, mut api: SandApi) {
    // rocket has complicated behavior that is staged piecewise in ra.
    // it would be awesome to diagram the ranges of values and their meaning

    if cell.rb == 0 {
        //initialize
        api.set(
            0,
            0,
            Cell {
                ra: 0,
                rb: 100,
                ..cell
            },
        );
        return;
    }

    let clone_species = if cell.rb != 100 {
        unsafe { mem::transmute(cell.rb as u8) }
    } else {
        Species::Sand
    };

    let (sx, sy) = api.rand_vec();
    let sample = api.get(sx, sy);

    if cell.rb == 100 //the type is unset
        && sample.species != Species::Empty
        && sample.species != Species::Rocket
        && sample.species != Species::Wall
        && sample.species != Species::Cloner
    {
        api.set(
            0,
            0,
            Cell {
                ra: 1,
                rb: sample.species as u8, //store the type
                ..cell
            },
        );
        return;
    }

    let ra = cell.ra;

    if ra == 0 {
        //falling (dormant)
        let dx = api.rand_dir();
        let nbr = api.get(0, 1);
        if nbr.species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(0, 1, cell);
        } else if api.get(dx, 1).species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(dx, 1, cell);
        } else if nbr.species == Species::Water
            || nbr.species == Species::Gas
            || nbr.species == Species::Oil
            || nbr.species == Species::Acid
        {
            api.set(0, 0, nbr);
            api.set(0, 1, cell);
        } else {
            api.set(0, 0, cell);
        }
    } else if ra == 1 {
        //launch
        api.set(0, 0, Cell { ra: 2, ..cell });
    } else if ra == 2 {
        let (mut dx, mut dy) = api.rand_vec_8();
        let nbr = api.get(dx, dy);
        if nbr.species != Species::Empty {
            dx *= -1;
            dy *= -1;
        }
        api.set(
            0,
            0,
            Cell {
                ra: 100 + join_dy_dx(dx, dy),
                ..cell
            },
        );
    } else if ra > 50 {
        let (dx, dy) = split_dy_dx(cell.ra - 100);

        let nbr = api.get(dx, dy * 2);

        if nbr.species == Species::Empty
            || nbr.species == Species::Fire
            || nbr.species == Species::Rocket
        {
            api.set(0, 0, Cell::new(clone_species));
            api.set(0, dy, Cell::new(clone_species));

            let (ndx, ndy) = match api.rand_int(100) % 5 {
                0 => adjacency_left((dx, dy)),
                1 => adjacency_right((dx, dy)),
                // 2 => adjacency_right((dx, dy)),
                _ => (dx, dy),
            };
            api.set(
                dx,
                dy * 2,
                Cell {
                    ra: 100 + join_dy_dx(ndx, ndy),
                    ..cell
                },
            );
        } else {
            //fizzle
            api.set(0, 0, EMPTY_CELL);
        }
    }
}

pub fn update_fire(cell: Cell, mut api: SandApi) {
    let ra = cell.ra;
    let mut degraded = cell.clone();
    degraded.ra = ra.saturating_sub((2 + api.rand_dir()) as u8);

    let (dx, dy) = api.rand_vec();

    api.set_fluid(Wind {
        dx: 0,
        dy: 150,
        pressure: 1,
        density: 120,
    });
    if api.get(dx, dy).species == Species::Gas || api.get(dx, dy).species == Species::Dust {
        api.set(
            dx,
            dy,
            Cell {
                species: Species::Fire,
                ra: (150 + (dx + dy) * 10) as u8,
                rb: 0,
                clock: 0,
            },
        );
        api.set_fluid(Wind {
            dx: 0,
            dy: 0,
            pressure: 80,
            density: 40,
        });
    }
    if ra < 5 || api.get(dx, dy).species == Species::Water {
        api.set(0, 0, EMPTY_CELL);
    } else if api.get(dx, dy).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, dy, degraded);
    } else {
        api.set(0, 0, degraded);
    }
}

pub fn update_lava(cell: Cell, mut api: SandApi) {
    api.set_fluid(Wind {
        dx: 0,
        dy: 10,
        pressure: 0,
        density: 60,
    });
    let (dx, dy) = api.rand_vec();

    if api.get(dx, dy).species == Species::Gas || api.get(dx, dy).species == Species::Dust {
        api.set(
            dx,
            dy,
            Cell {
                species: Species::Fire,
                ra: (150 + (dx + dy) * 10) as u8,
                rb: 0,
                clock: 0,
            },
        );
    }
    let sample = api.get(dx, dy);
    if sample.species == Species::Water {
        api.set(
            0,
            0,
            Cell {
                species: Species::Stone,
                ra: (150 + (dx + dy) * 10) as u8,
                rb: 0,
                clock: 0,
            },
        );
        api.set(dx, dy, EMPTY_CELL);
    } else if api.get(0, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if api.get(dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else {
        api.set(0, 0, cell);
    }
}

pub fn update_wood(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;

    let (dx, dy) = api.rand_vec();

    let nbr_species = api.get(dx, dy).species;
    if rb == 0 && nbr_species == Species::Fire || nbr_species == Species::Lava {
        api.set(
            0,
            0,
            Cell {
                species: Species::Wood,
                ra: cell.ra,
                rb: 90,
                clock: 0,
            },
        );
    }

    if rb > 1 {
        api.set(
            0,
            0,
            Cell {
                species: Species::Wood,
                ra: cell.ra,
                rb: rb - 1,
                clock: 0,
            },
        );

        if rb % 4 == 0 && nbr_species == Species::Empty {
            let ra = 30 + api.rand_int(60) as u8;
            api.set(
                dx,
                dy,
                Cell {
                    species: Species::Fire,
                    ra,
                    rb: 0,
                    clock: 0,
                },
            )
        }
        if nbr_species == Species::Water {
            api.set(
                0,
                0,
                Cell {
                    species: Species::Wood,
                    ra: 50,
                    rb: 0,
                    clock: 0,
                },
            );
            api.set_fluid(Wind {
                dx: 0,
                dy: 0,
                pressure: 0,
                density: 220,
            });
        }
    } else if rb == 1 {
        api.set(
            0,
            0,
            Cell {
                species: Species::Empty,
                ra: cell.ra,
                rb: 90,
                clock: 0,
            },
        );
    }
}
pub fn update_ice(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();

    let i = api.rand_int(100);

    let fluid = api.get_fluid();

    if fluid.pressure > 120 && api.rand_int(1) == 0 {
        api.set(
            0,
            0,
            Cell {
                species: Species::Water,
                ra: cell.ra,
                rb: 0,
                clock: 0,
            },
        );
        return;
    }

    let nbr_species = api.get(dx, dy).species;
    if nbr_species == Species::Fire || nbr_species == Species::Lava {
        api.set(
            0,
            0,
            Cell {
                species: Species::Water,
                ra: cell.ra,
                rb: cell.rb,
                clock: 0,
            },
        );
    } else if nbr_species == Species::Water && i < 7 {
        api.set(
            dx,
            dy,
            Cell {
                species: Species::Ice,
                ra: cell.ra,
                rb: cell.rb,
                clock: 0,
            },
        );
    }
}

pub fn update_plant(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;

    let mut i = api.rand_int(100);
    let (dx, dy) = api.rand_vec();

    let nbr_species = api.get(dx, dy).species;
    if rb == 0 && nbr_species == Species::Fire || nbr_species == Species::Lava {
        api.set(
            0,
            0,
            Cell {
                species: Species::Plant,
                ra: cell.ra,
                rb: 20,
                clock: 0,
            },
        );
    }
    if nbr_species == Species::Wood {
        let (dx, dy) = api.rand_vec();

        let drift = (i % 15) - 7;
        let newra = (cell.ra as i32 + drift) as u8;
        if api.get(dx, dy).species == Species::Empty {
            api.set(
                dx,
                dy,
                Cell {
                    species: Species::Plant,
                    ra: newra,
                    rb: 0,
                    clock: 0,
                },
            );
        }
    }
    if api.rand_int(100) > 80
        && (nbr_species == Species::Water
            || nbr_species == Species::Fungus
                && (api.get(-dx, dy).species == Species::Empty
                    || api.get(-dx, dy).species == Species::Water
                    || api.get(-dx, dy).species == Species::Fungus))
    {
        i = api.rand_int(100);
        let drift = (i % 15) - 7;
        let newra = (cell.ra as i32 + drift) as u8;
        api.set(
            dx,
            dy,
            Cell {
                ra: newra,
                rb: 0,
                ..cell
            },
        );
        api.set(-dx, dy, EMPTY_CELL);
    }

    if rb > 1 {
        api.set(
            0,
            0,
            Cell {
                ra: cell.ra,
                rb: rb - 1,
                ..cell
            },
        );

        if nbr_species == Species::Empty {
            let ra = 20 + api.rand_int(30) as u8;
            api.set(
                dx,
                dy,
                Cell {
                    species: Species::Fire,
                    ra,
                    rb: 0,
                    clock: 0,
                },
            );
        }
        if nbr_species == Species::Water {
            api.set(
                0,
                0,
                Cell {
                    ra: 50,
                    rb: 0,
                    ..cell
                },
            )
        }
    } else if rb == 1 {
        api.set(0, 0, EMPTY_CELL);
    }
    let ra = cell.ra;
    if ra > 50
        && api.get(1, 1).species != Species::Plant
        && api.get(-1, 1).species != Species::Plant
    {
        if api.get(0, 1).species == Species::Empty {
            let i = (js_sys::Math::random() * js_sys::Math::random() * 100.) as i32;
            let dec = api.rand_int(30) - 20;
            if (i + ra as i32) > 165 {
                api.set(
                    0,
                    1,
                    Cell {
                        ra: (ra as i32 + dec) as u8,
                        ..cell
                    },
                );
            }
        } else {
            api.set(
                0,
                0,
                Cell {
                    ra: (ra - 1) as u8,
                    ..cell
                },
            );
        }
    }
}

pub fn update_seed(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;
    let ra = cell.ra;

    let (dx, dy) = api.rand_vec();

    let nbr_species = api.get(dx, dy).species;
    if nbr_species == Species::Fire || nbr_species == Species::Lava {
        api.set(
            0,
            0,
            Cell {
                species: Species::Fire,
                ra: 5,
                rb: 0,
                clock: 0,
            },
        );
        return;
    }

    if rb == 0 {
        //falling

        let dxf = api.rand_dir(); //falling dx
        let nbr_species_below = api.get(dxf, 1).species;
        if nbr_species_below == Species::Sand
            || nbr_species_below == Species::Plant
            || nbr_species_below == Species::Fungus
        {
            let rb = (api.rand_int(253) + 1) as u8;
            api.set(0, 0, Cell { rb, ..cell });
            return;
        }

        let nbr = api.get(0, 1);
        if nbr.species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(0, 1, cell);
        } else if api.get(dxf, 1).species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(dxf, 1, cell);
        } else if nbr.species == Species::Water
            || nbr.species == Species::Gas
            || nbr.species == Species::Oil
            || nbr.species == Species::Acid
        {
            api.set(0, 0, nbr);
            api.set(0, 1, cell);
        } else {
            api.set(0, 0, cell);
        }
    } else {
        if ra > 60 {
            //stem
            let dxr = api.rand_dir(); //raising dx
            if api.rand_int(100) > 75 {
                if (api.get(dxr, -1).species == Species::Empty
                    || api.get(dxr, -1).species == Species::Sand
                    || api.get(dxr, -1).species == Species::Seed)
                    && api.get(1, -1).species != Species::Plant
                    && api.get(-1, -1).species != Species::Plant
                {
                    let ra = (ra as i32 - api.rand_int(10)) as u8;
                    api.set(dxr, -1, Cell { ra, ..cell });
                    let ra2 = 80 + api.rand_int(30) as u8;
                    api.set(
                        0,
                        0,
                        Cell {
                            species: Species::Plant,
                            ra: ra2,
                            rb: 0,
                            clock: 0,
                        },
                    )
                } else {
                    api.set(0, 0, EMPTY_CELL);
                }
            }
        } else {
            if ra > 40 {
                //petals

                let (mdx, mdy) = api.rand_vec();

                let (ldx, ldy) = adjacency_left((mdx, mdy));
                let (rdx, rdy) = adjacency_right((mdx, mdy));

                if (api.get(mdx, mdy).species == Species::Empty
                    || api.get(mdx, mdy).species == Species::Plant)
                    && (api.get(ldx, ldy).species == Species::Empty
                        || api.get(rdx, rdy).species == Species::Empty)
                {
                    let i = (js_sys::Math::random() * js_sys::Math::random() * 100.) as i32;
                    let dec = 9 - api.rand_int(3);
                    if (i + ra as i32) > 100 {
                        api.set(
                            mdx,
                            mdy,
                            Cell {
                                ra: (ra as i32 - dec) as u8,
                                ..cell
                            },
                        );
                    }
                }
            } else {
                if nbr_species == Species::Water {
                    api.set(dx, dy, Cell::new(Species::Seed))
                }
            }
        }
    }
}

pub fn update_fungus(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;

    let (dx, dy) = api.rand_vec();

    let nbr_species = api.get(dx, dy).species;
    if rb == 0 && nbr_species == Species::Fire || nbr_species == Species::Lava {
        api.set(
            0,
            0,
            Cell {
                species: Species::Fungus,
                ra: cell.ra,
                rb: 10,
                clock: 0,
            },
        );
    }
    let mut i = api.rand_int(100);

    if nbr_species != Species::Empty
        && nbr_species != Species::Fungus
        && nbr_species != Species::Fire
        && nbr_species != Species::Ice
    {
        let (dx, dy) = api.rand_vec();

        let drift = (i % 15) - 7;
        let newra = (cell.ra as i32 + drift) as u8;
        if api.get(dx, dy).species == Species::Empty {
            api.set(
                dx,
                dy,
                Cell {
                    species: Species::Fungus,
                    ra: newra,
                    rb: 0,
                    clock: 0,
                },
            );
        }
    }

    if i > 9
        && nbr_species == Species::Wood
        && api.get(-dx, dy).species == Species::Wood
        && api.get(dx, -dy).species == Species::Wood
        && api.get(dx, dy).ra % 4 != 0
    {
        i = api.rand_int(100);
        let drift = (i % 15) - 7;
        let newra = (cell.ra as i32 + drift) as u8;
        api.set(
            dx,
            dy,
            Cell {
                ra: newra,
                rb: 0,
                ..cell
            },
        );
    }

    if rb > 1 {
        api.set(
            0,
            0,
            Cell {
                ra: cell.ra,
                rb: rb - 1,
                ..cell
            },
        );
        if nbr_species == Species::Empty {
            let ra = 10 + api.rand_int(10) as u8;
            api.set(
                dx,
                dy,
                Cell {
                    species: Species::Fire,
                    ra,
                    rb: 0,
                    clock: 0,
                },
            )
        }
        if nbr_species == Species::Water {
            api.set(
                0,
                0,
                Cell {
                    ra: 50,
                    rb: 0,
                    ..cell
                },
            )
        }
    } else if rb == 1 {
        api.set(0, 0, EMPTY_CELL);
    }

    let ra = cell.ra;

    if ra > 120 {
        let (mdx, mdy) = api.rand_vec();

        let (ldx, ldy) = adjacency_left((mdx, mdy));
        let (rdx, rdy) = adjacency_right((mdx, mdy));
        if api.get(mdx, mdy).species == Species::Empty
            && api.get(ldx, ldy).species != Species::Fungus
            && api.get(rdx, rdy).species != Species::Fungus
        {
            let i = (js_sys::Math::random() * js_sys::Math::random() * 100.) as i32;
            let dec = 15 - api.rand_int(20);
            if (i + ra as i32) > 165 {
                api.set(
                    mdx,
                    mdy,
                    Cell {
                        ra: (ra as i32 - dec) as u8,
                        ..cell
                    },
                );
            }
        }
    }
}

pub fn update_acid(cell: Cell, mut api: SandApi) {
    let dx = api.rand_dir();

    let ra = cell.ra;
    let mut degraded = cell.clone();
    degraded.ra = ra.saturating_sub(60);
    // i = api.rand_int(100);
    if degraded.ra < 80 {
        degraded = EMPTY_CELL;
    }
    if api.get(0, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else if api.get(-dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 0, cell);
    } else {
        if api.get(0, 1).species != Species::Wall && api.get(0, 1).species != Species::Acid {
            api.set(0, 0, EMPTY_CELL);
            api.set(0, 1, degraded);
        } else if api.get(dx, 0).species != Species::Wall && api.get(dx, 0).species != Species::Acid
        {
            api.set(0, 0, EMPTY_CELL);
            api.set(dx, 0, degraded);
        } else if api.get(-dx, 0).species != Species::Wall
            && api.get(-dx, 0).species != Species::Acid
        {
            api.set(0, 0, EMPTY_CELL);
            api.set(-dx, 0, degraded);
        } else if api.get(0, -1).species != Species::Wall
            && api.get(0, -1).species != Species::Acid
            && api.get(0, -1).species != Species::Empty
        {
            api.set(0, 0, EMPTY_CELL);
            api.set(0, -1, degraded);
        } else {
            api.set(0, 0, cell);
        }
    }
}

pub fn update_mite(cell: Cell, mut api: SandApi) {
    let mut i = api.rand_int(100);
    let mut dx = 0;
    if cell.ra < 20 {
        dx = (cell.ra as i32) - 1;
    }
    let mut dy = 1;
    let mut mite = cell.clone();

    if cell.rb > 10 {
        // /
        mite.rb = mite.rb.saturating_sub(1);
        dy = -1;
    } else if cell.rb > 1 {
        // \
        mite.rb = mite.rb.saturating_sub(1);
    } else {
        // |
        dx = 0;
    }
    let nbr = api.get(dx, dy);

    let sx = (i % 3) - 1;
    i = api.rand_int(1000);
    let sy = (i % 3) - 1;
    let sample = api.get(sx, sy).species;
    if sample == Species::Fire
        || sample == Species::Lava
        || sample == Species::Water
        || sample == Species::Oil
    {
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    if (sample == Species::Plant || sample == Species::Wood || sample == Species::Seed) && i > 800 {
        api.set(0, 0, EMPTY_CELL);
        api.set(sx, sy, cell);

        return;
    }
    if sample == Species::Dust {
        api.set(sx, sy, if i > 800 { cell } else { EMPTY_CELL });
    }

    if nbr.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, dy, mite);
    } else if dy == 1 && i > 800 {
        i = api.rand_int(100);
        let mut ndx = (i % 3) - 1;
        if i < 6 {
            //switch direction
            ndx = dx;
        }

        mite.ra = (1 + ndx) as u8;
        mite.rb = 10 + (i % 10) as u8; //hop height

        api.set(0, 0, mite);
    } else {
        if api.get(-1, 0).species == Species::Mite
            && api.get(1, 0).species == Species::Mite
            && api.get(0, -1).species == Species::Mite
        {
            api.set(0, 0, EMPTY_CELL);
        } else {
            if api.get(0, 1).species == Species::Ice {
                if api.get(dx, 0).species == Species::Empty {
                    api.set(0, 0, EMPTY_CELL);
                    api.set(dx, 0, mite);
                }
            } else {
                api.set(0, 0, mite);
            }
        }
    }
}
