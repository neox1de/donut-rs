use std::f32::consts::PI;
use std::io;
use std::thread;
use std::time::Duration;

const DONUT_THICKNESS: f32 = 0.5;
const DONUT_RADIUS: f32 = 0.25;
const VIEWER_DISTANCE: f32 = 3.0;
const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 40;
const PROJECTION_CONSTANT: f32 = SCREEN_WIDTH as f32 * VIEWER_DISTANCE * 3.0 / (8.0 * (DONUT_THICKNESS + DONUT_RADIUS));

fn main() -> io::Result<()> {
    let mut angle_a: f32 = 0.0;
    let mut angle_b: f32 = 0.0;
    let luminance_chars = ".,-~:;=!*#$@".chars().collect::<Vec<_>>();
    let light_source = (0.0f32, 1.0f32, -1.0f32);
    let light_distance = (light_source.0.powi(2) + light_source.1.powi(2) + light_source.2.powi(2)).sqrt();
    
    loop {
        let (cos_a, sin_a) = (angle_a.cos(), angle_a.sin());
        let (cos_b, sin_b) = (angle_b.cos(), angle_b.sin());

        let mut pixel_matrix = vec![vec![' '; SCREEN_WIDTH]; SCREEN_HEIGHT];
        let mut depth_buffer = vec![vec![f32::MAX; SCREEN_WIDTH]; SCREEN_HEIGHT];

        let mut theta = 0.0;
        while theta < 2.0 * PI {
            let (cos_theta, sin_theta) = (theta.cos(), theta.sin());
            let mut phi = 0.0;
            while phi < 2.0 * PI {
                let (cos_phi, sin_phi) = (phi.cos(), phi.sin());

                let donut_x = (DONUT_THICKNESS + DONUT_RADIUS * cos_theta) * cos_phi;
                let donut_y = (DONUT_THICKNESS + DONUT_RADIUS * cos_theta) * sin_phi;
                let donut_z = DONUT_RADIUS * sin_theta;

                let rotated_y = donut_y * cos_a - donut_z * sin_a;
                let rotated_z = donut_y * sin_a + donut_z * cos_a;
                let final_x = donut_x * cos_b + rotated_z * sin_b;
                let final_z = -donut_x * sin_b + rotated_z * cos_b;

                let z_distance = final_z + VIEWER_DISTANCE;
                let inverse_z = 1.0 / z_distance;
                let screen_x = (PROJECTION_CONSTANT * inverse_z * final_x) as i32;
                let screen_y = (PROJECTION_CONSTANT * inverse_z * rotated_y) as i32;

                let pixel_x = ((SCREEN_WIDTH as f32 / 2.0) + screen_x as f32) as usize;
                let pixel_y = ((SCREEN_HEIGHT as f32 / 2.0) + screen_y as f32) as usize;

                if pixel_x < SCREEN_WIDTH && pixel_y < SCREEN_HEIGHT {
                    let normal = (cos_theta * cos_phi, cos_theta * sin_phi, sin_theta);
                    
                    let rotated_ny = normal.1 * cos_a - normal.2 * sin_a;
                    let rotated_nz = normal.1 * sin_a + normal.2 * cos_a;
                    let final_nx = normal.0 * cos_b + rotated_nz * sin_b;
                    let final_nz = -normal.0 * sin_b + rotated_nz * cos_b;

                    let luminance = (final_nx * light_source.0 + rotated_ny * light_source.1 + final_nz * light_source.2) / light_distance;

                    if luminance > 0.0 {
                        let char_index = ((luminance * (luminance_chars.len() - 1) as f32).round()) as usize;

                        if z_distance < depth_buffer[pixel_y][pixel_x] {
                            depth_buffer[pixel_y][pixel_x] = z_distance;
                            pixel_matrix[pixel_y][pixel_x] = luminance_chars[char_index];
                        }
                    }
                }
                phi += 0.02;
            }
            theta += 0.07;
        }
        print!("\x1b[2J\x1b[H");
        for row in pixel_matrix {
            println!("{}", row.iter().collect::<String>());
        }
        thread::sleep(Duration::from_millis(50));

        angle_a += 0.07;
        angle_b += 0.03;
    }
}
