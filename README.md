<img src="qoqo_Logo_vertical_color.png" alt="qoqo logo" width="300" />

[Here](https://hqsquantumsimulations.github.io/qoqo-qiskit) you can find links leading to the single packages' docs.

# qoqo_qiskit
[![PyPI](https://img.shields.io/pypi/v/qoqo_qiskit)](https://pypi.org/project/qoqo_qiskit/)
[![Documentation Status](https://img.shields.io/badge/docs-documentation-green)](https://hqsquantumsimulations.github.io/qoqo-qiskit/qoqo_qiskit_api/html/index.html)

Qiskit interface for the qoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

### Installation

We provide pre-built binaries for linux, macos and windows on x86_64 hardware and macos on arm64. Simply install the pre-built wheels with

```shell
pip install qoqo-qiskit
```

## Testing

This software is still in the beta stage. Functions and documentation are not yet complete and breaking changes can occur.

If you find unexpected behaviour please open a github issue. You can also run the pytests in qoqo_qiskit/tests/ locally.


# qoqo_qiskit_devices
[![PyPI](https://img.shields.io/pypi/v/qoqo_qiskit_devices)](https://pypi.org/project/qoqo_qiskit_devices/)
[![Documentation Status](https://img.shields.io/badge/docs-documentation-green)](https://hqsquantumsimulations.github.io/qoqo-qiskit/qoqo_qiskit_devices_api/html/index.html)
![Crates.io](https://img.shields.io/crates/l/qoqo-qiskit-devices)

Qiskit devices python interface for the qoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

In order to make the update a device instance with Qiskit's information possible, the user has to run the following code before using this package:
```python
from qiskit_ibm_provider import IBMProvider

IBMProvider.save_account(token=MY_API_TOKEN)
```
Where `MY_API_TOKEN` is the API key that can be found in the account settings of the IBM Quantum website after registration.

# roqoqo_qiskit_devices
[![Crates.io](https://img.shields.io/crates/v/roqoqo-qiskit-devices)](https://crates.io/crates/roqoqo-qiskit-devices)
![Crates.io](https://img.shields.io/crates/l/roqoqo-qiskit-devices)

Qiskit devices Rust interface for the qoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

### Installation

To use roqoqo_qiskit_devices in a Rust project simply add

```TOML
roqoqo_qiskit_devices = {version="0.1"}
```

to the `[dependencies]` section of the project Cargo.toml.

## General Notes

Qiskit is under the Apache-2.0 license ( see https://github.com/Qiskit/qiskit/blob/master/LICENSE.txt ).

qoqo_qiskit, qoqo_qiskit_devices and roqoqo_qiskit_devices are also provided under the Apache-2.0 license.

This project has been partly supported by [QSolid](https://www.q-solid.de/).