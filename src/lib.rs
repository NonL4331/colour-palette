enum Channel {
    Red,
    Green,
    Blue,
}

type Colour = [u8; 3];

trait ColourTrait {
    fn new(r: u8, g: u8, b: u8) -> Self;

    fn max_by_component(&mut self, other: &Self);

    fn min_by_component(&self, other: &Self) -> Self;

    fn get_highest_channel(&self) -> Channel;

    fn get_channel_value(&self, channel: &Channel) -> u8;
}

impl ColourTrait for Colour {
    fn new(r: u8, g: u8, b: u8) -> Self {
        [r, g, b]
    }

    fn max_by_component(&mut self, other: &Self) {
        self[0] = std::cmp::max(self[0], other[0]);
        self[1] = std::cmp::max(self[1], other[1]);
        self[2] = std::cmp::max(self[2], other[2]);
    }

    fn min_by_component(&self, other: &Self) -> Self {
        Colour::new(
            std::cmp::min(self[0], other[0]),
            std::cmp::min(self[1], other[1]),
            std::cmp::min(self[2], other[2]),
        )
    }

    fn get_highest_channel(&self) -> Channel {
        if self[0] > self[1] && self[0] > self[2] {
            Channel::Red
        } else if self[1] > self[2] {
            Channel::Green
        } else {
            Channel::Blue
        }
    }

    fn get_channel_value(&self, channel: &Channel) -> u8 {
        match channel {
            Channel::Red => self[0],
            Channel::Green => self[1],
            Channel::Blue => self[2],
        }
    }
}

fn get_channel_mean(pixels: &[Colour], channel: &Channel) -> u8 {
    let mut sum: u64 = 0;
    for pixel in pixels {
        sum += pixel.get_channel_value(&channel) as u64;
    }
    (sum / pixels.len() as u64) as u8
}

fn get_channel_ranges(pixels: &[Colour]) -> Colour {
    let mut min: Option<Colour> = None;
    let mut max: Option<Colour> = None;
    for pixel in pixels {
        match min {
            None => {
                min = Some(*pixel);
            }
            Some(min) => {
                min.min_by_component(pixel);
            }
        }
        match max {
            None => {
                max = Some(*pixel);
            }
            Some(mut max) => {
                max.max_by_component(pixel);
            }
        }
    }
    let min = min.unwrap();
    let max = max.unwrap();

    Colour::new(max[0] - min[0], max[1] - min[1], max[2] - min[2])
}

fn get_mean(pixels: &[Colour]) -> Colour {
    let mut sum: [u64; 3] = [0; 3];
    for pixel in pixels {
        sum[0] += pixel[0] as u64;
        sum[1] += pixel[1] as u64;
        sum[2] += pixel[2] as u64;
    }
    let num_pixels = pixels.len() as u64;
    sum[0] /= num_pixels;
    sum[1] /= num_pixels;
    sum[2] /= num_pixels;
    Colour::new(sum[0] as u8, sum[1] as u8, sum[2] as u8)
}

pub fn generate_palette(mut image: Vec<Colour>, num_colours: u8) -> Vec<Colour> {
    let mut buckets = vec![image.as_mut_slice()];

    let mut len = buckets.len();

    while len < num_colours as usize {
        len = buckets.len();
        for _ in 0..len {
            let bucket = buckets.remove(0);

            let range = get_channel_ranges(&bucket);
            let largest_channel = range.get_highest_channel();
            let mean = get_channel_mean(&bucket, &largest_channel);

            bucket.sort_by(|a, b| {
                a.get_channel_value(&largest_channel)
                    .cmp(&b.get_channel_value(&largest_channel))
            });

            let mean_index = match bucket
                .iter()
                .position(|&pixel| pixel.get_channel_value(&largest_channel) > mean)
            {
                Some(val) => val,
                None => 0,
            };

            let (left, right) = bucket.split_at_mut(mean_index);
            buckets.push(left);
            buckets.push(right);
        }
    }

    let mut palette = Vec::new();

    for bucket in buckets {
        palette.push(get_mean(bucket));
    }

    palette
}

#[cfg(test)]
mod tests {
    extern crate image;
    use super::*;
    use image::Pixel;

    #[test]
    fn test_image() {
        let mut image = Vec::new();
        for (_, _, pixel) in image::open("test.png")
            .unwrap()
            .into_rgb8()
            .enumerate_pixels()
        {
            let rgb = pixel.to_rgb();
            image.push(Colour::new(rgb[0], rgb[1], rgb[2]));
        }
        let palette = generate_palette(image, 16);

        for color in palette {
            println!("{:?}", color);
        }
    }
}
