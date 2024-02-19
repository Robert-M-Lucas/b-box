#[macro_export]
macro_rules! b {
    ($($tts:tt)*) => { Box::new($($tts)*) };
}