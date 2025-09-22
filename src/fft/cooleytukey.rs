// cooley tukey fft implementation with metal API rust bindings
// reference: https://github.com/ShoYamanishi/AppleNumericalComputing/tree/main/13_fft

trait CooleyTukey {
    async fn crude_alg<T>(&self, input: &mut Vec<T>, output: &mut Vec<T>);
}
