# aviod conda 's env conflact with python
unset CONDA_PREFIX
virtualenv rust-json
source rust-json/bin/activate
cd ./rust_json
maturin develop
python

# if you want to use rust_json module in py
import rust_json
