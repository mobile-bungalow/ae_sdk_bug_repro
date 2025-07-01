use after_effects as ae;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Params {}

#[derive(Default)]
struct Plugin {}

// Uncomment me to fix the bug!
//
// impl AdobePluginInstance for u32 {
//     fn flatten(&self) -> Result<(u16, Vec<u8>), Error> {
//         Ok((
//             std::mem::size_of::<u32>() as u16,
//             self.to_be_bytes().to_vec(),
//         ))
//     }
//     fn unflatten(_: u16, i: &[u8]) -> Result<Self, Error> {
//         Ok(Self::from_be_bytes([i[0], i[1], i[2], i[3]]))
//     }
//
//     fn render(&self, _: &mut PluginState, _: &Layer, _: &mut Layer) -> Result<(), ae::Error> {
//         Ok(())
//     }
//     fn handle_command(&mut self, _: &mut PluginState, _: Command) -> Result<(), Error> {
//         Ok(())
//     }
//
//     fn do_dialog(&mut self, _: &mut PluginState) -> Result<(), ae::Error> {
//         Ok(())
//     }
// }
// ae::define_effect!(Plugin, u32, Params);

// also comment me out
ae::define_effect!(Plugin, (), Params);

impl AdobePluginGlobal for Plugin {
    fn can_load(_host_name: &str, _host_version: &str) -> bool {
        true
    }

    fn params_setup(
        &self,
        params: &mut ae::Parameters<Params>,
        _: ae::InData,
        _: ae::OutData,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn handle_command(
        &mut self,
        cmd: ae::Command,
        in_data: ae::InData,
        _: ae::OutData,
        _: &mut ae::Parameters<Params>,
    ) -> Result<(), ae::Error> {
        match cmd {
            ae::Command::DoDialog => {}
            ae::Command::Render {
                in_layer,
                mut out_layer,
            } => {
                let extent_hint = in_data.extent_hint();

                in_layer.iterate_with(
                    &mut out_layer,
                    0,
                    extent_hint.height(),
                    Some(extent_hint),
                    |_x: i32,
                     _y: i32,
                     pixel: ae::GenericPixel,
                     out_pixel: ae::GenericPixelMut|
                     -> Result<(), Error> {
                        match (pixel, out_pixel) {
                            (
                                ae::GenericPixel::Pixel8(pixel),
                                ae::GenericPixelMut::Pixel8(out_pixel),
                            ) => {
                                *out_pixel = *pixel;
                            }
                            (
                                ae::GenericPixel::Pixel16(pixel),
                                ae::GenericPixelMut::Pixel16(out_pixel),
                            ) => {
                                *out_pixel = *pixel;
                            }
                            _ => return Err(Error::BadCallbackParameter),
                        }
                        Ok(())
                    },
                )?;
            }
            _ => {}
        }
        Ok(())
    }
}
