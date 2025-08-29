use minifb::{Key, Window, WindowOptions};
use std::io;
use std::{env, fs};

// Constants
static FOV: f64 = std::f64::consts::PI / 3.0;

static H: usize = 120;
static W: usize = 240;
static K: f64 = H as f64;

static SCALE: usize = 10;
static WINDOW_HEIGHT: usize = H * SCALE;
static WINDOW_WIDTH: usize = (W + 10) * SCALE;


fn load_map(path: &std::path::Path) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(path).expect("Load");
    let map = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    map
}

fn print_map(
    map: &[Vec<char>],
    player_x: f64,
    player_y: f64,
    player_angle: f64,
    start_row: usize,
    screen: &mut [Vec<char>],
) {
    for (map_row, row) in map.iter().enumerate() {
        for (map_col, &c) in row.iter().enumerate() {
            screen[map_row + start_row][map_col] = c;
        }
    }

    let x_str = format!("Player x: {:.2}", player_x);
    let y_str = format!("Player y: {:.2}", player_y);
    let angle_str = format!("Player angle: {:.2}", player_angle);

    let info_start_row = start_row + map.len();
    for (i, s) in [x_str, y_str, angle_str].iter().enumerate() {
        let row = info_start_row + i;
        for (col, c) in s.chars().enumerate() {
            if col < screen[row].len() {
                screen[row][col] = c;
            }
        }
    }
}

fn cast_ray(x: f64, y: f64, ray_angle: f64, map: &[Vec<char>]) -> f64 {
    let step = 0.1;

    let max_distance = 60.0;
    let mut distance = 0.001;

    while distance < max_distance {
        let ray_x = x + distance * ray_angle.cos();
        let ray_y = y + distance * ray_angle.sin();

        let map_x = ray_x.floor() as usize;
        let map_y = ray_y.floor() as usize;

        if map_y < map.len() && map_x < map[0].len() && map[map_y][map_x] == '#' {
            return distance;
        }
        distance += step;
    }
    max_distance
}

fn calculate_distances(
    player_x: f64,
    player_y: f64,
    player_angle: f64,
    map: &[Vec<char>],
) -> Vec<f64> {
    let mut distances = vec![0.0; W];

    for (col, distance) in distances.iter_mut().enumerate().take(W) {
        let ray_angle = player_angle - FOV / 2.0 + (col as f64 * FOV / W as f64);
        *distance = cast_ray(player_x, player_y, ray_angle, map);
    }

    distances
}

fn load_player(map: &[Vec<char>]) -> Result<(f64, f64), std::io::Error> {
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == 'P' {
                return Ok((col_idx as f64 + 0.5, row_idx as f64 + 0.5));
            }
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "No player starting position 'P' found in map",
    ))
}

fn render_screen(
    player_angle: f64,
    map: &[Vec<char>],
    player_x: f64,
    player_y: f64,
    buffer: &mut Vec<u32>,
    screen_height: usize,
    screen_width: usize,
) {
    // Clear buffer
    buffer.fill(0x000000);

    let distances = calculate_distances(player_x, player_y, player_angle, map);

    // Calculate wall height for each screen column
    let mut wall_heights = vec![0; W];
    for col_idx in 0..wall_heights.len() {
        wall_heights[col_idx] = (K / distances[col_idx]).max(0.0).min(H as f64) as usize;
    }

    for col in 0..W {
        let wall_height = wall_heights[col];
        let distance = distances[col];

        let color = if distance < 2.0 {
            // Wall is close
            0xFFFFFF
        } else if distance < 4.0 {
            0xAAAAAA
        } else {
            0x555555
        };

        for row in (H / 2).saturating_sub(wall_height / 2)..(H / 2 + wall_height / 2).min(H) {
            for x in (col * SCALE)..((col + 1) * SCALE) {
                for y in (row * SCALE)..((row + 1) * SCALE) {
                    if y < screen_height && x < screen_width {
                        buffer[y * screen_width + x] = color;
                    }
                }
            }
        }
    }

    /*
    for (screen_row_idx, screen_row) in screen.iter_mut().enumerate().take(H) {
        for screen_col_idx in 0..W {
            if ((H / 2) - (wall_heights[screen_col_idx] / 2)) < screen_row_idx
                && ((H / 2) + (wall_heights[screen_col_idx] / 2)) > screen_row_idx
            {
                // Paint a wall here
                let d = distances[screen_col_idx];
                /*
                if d < 2.0 {
                    screen_row[screen_col_idx] = '█';
                } else if d < 4.0 {
                    screen_row[screen_col_idx] = '▓';
                } else {
                    screen_row[screen_col_idx] = '▒';
                }
                */
                //buffer[
            }
        }
    }
    */

    //print_map(map, player_x, player_y, player_angle, H, &mut screen);

    //screen
}

/*
fn display_screen(screen: &[Vec<char>], prev_screen: &[Vec<char>]) -> std::io::Result<()> {
    let mut stdout = stdout();

    for (row_index, row) in screen.iter().enumerate() {
        if *row != prev_screen[row_index] {
            stdout.queue(MoveTo(0, row_index as u16))?;
            let line: String = row.iter().collect();
            stdout.queue(Print(&line))?;
            stdout.queue(Clear(ClearType::UntilNewLine))?;
        }
    }
    stdout.flush()?;
    Ok(())
}
*/

fn update_map(map: &mut [Vec<char>], player_x: f64, player_y: f64) {
    let map_y = (player_y.floor() as usize).min(map.len() - 1);
    let map_x = (player_x.floor() as usize).min(map[0].len() - 1);
    if map[map_y][map_x] != '#' {
        map[map_y][map_x] = 'P';
    }

    for row_idx in (map_y - 1)..(map_y + 2) {
        for col_idx in (map_x - 1)..(map_x + 2) {
            if (row_idx < map.len())
                && (col_idx < map[0].len())
                && !(row_idx == map_y && col_idx == map_x)
                && (map[row_idx][col_idx] == 'P')
            {
                map[row_idx][col_idx] = ' ';
            }
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: ./raytracing <PATH-TO-MAP>");
        std::process::exit(1);
    }

    let map_path = &std::path::Path::new(&args[1]);
    let mut map = load_map(map_path);

    let (mut player_x, mut player_y) = load_player(&map)?;
    let mut player_angle = std::f64::consts::PI / 2.0;

    let mut window = Window::new(
        "Raycasting",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default()
    ).expect("Couldn't create new window.");

    //window.limit_update_rate(Some(time::Duration::from_millis(16)));
    window.set_target_fps(60);

    let mut buffer: Vec<u32> = vec![0; WINDOW_HEIGHT * WINDOW_WIDTH];

    while window.is_open() && !window.is_key_down(Key::Q) {

        if window.is_key_down(Key::A) {
            player_angle -= 0.1;
        }
        if window.is_key_down(Key::D) {
            player_angle += 0.1;
        }

        if window.is_key_down(Key::W) {
            let new_x = player_x + 0.1 * player_angle.cos();
            let new_y = player_y + 0.1 * player_angle.sin();
            if map[new_y.floor() as usize][new_x.floor() as usize] != '#' {
                player_x = new_x;
                player_y = new_y;
            }
        }

        if window.is_key_down(Key::S) {
            let new_x = player_x - 0.1 * player_angle.cos();
            let new_y = player_y - 0.1 * player_angle.sin();
            if map[new_y.floor() as usize][new_x.floor() as usize] != '#' {
                player_x = new_x;
                player_y = new_y;
            }
        }

        update_map(&mut map, player_x, player_y);

        render_screen(
            player_angle,
            &map,
            player_x,
            player_y,
            &mut buffer,
            WINDOW_HEIGHT,
            WINDOW_WIDTH,
        );

        window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT).expect("Couldn't update buffer");
    }

    Ok(())
}
