// Copyright © 2023 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.

//! IBM Devices
//!
//! Provides the devices that are used to execute quantum programs on IBM's devices.

use roqoqo::devices::Device;

mod ibm_belem;
pub use crate::devices::ibm_belem::IBMBelemDevice;

mod ibm_jakarta;
pub use crate::devices::ibm_jakarta::IBMJakartaDevice;

mod ibm_lagos;
pub use crate::devices::ibm_lagos::IBMLagosDevice;

mod ibm_lima;
pub use crate::devices::ibm_lima::IBMLimaDevice;

mod ibm_manila;
pub use crate::devices::ibm_manila::IBMManilaDevice;

mod ibm_nairobi;
pub use crate::devices::ibm_nairobi::IBMNairobiDevice;

mod ibm_oslo;
pub use crate::devices::ibm_oslo::IBMOsloDevice;

mod ibm_perth;
pub use crate::devices::ibm_perth::IBMPerthDevice;

mod ibm_quito;
pub use crate::devices::ibm_quito::IBMQuitoDevice;

/// Collection of IBM quantum devices.
///
pub enum IBMDevice {
    IBMLagosDevice(IBMLagosDevice),
    IBMNairobiDevice(IBMNairobiDevice),
    IBMOsloDevice(IBMOsloDevice),
    IBMPerthDevice(IBMPerthDevice),
    IBMBelemDevice(IBMBelemDevice),
    IBMJakartaDevice(IBMJakartaDevice),
    IBMLimaDevice(IBMLimaDevice),
    IBMManilaDevice(IBMManilaDevice),
    IBMQuitoDevice(IBMQuitoDevice),
}

/// Implements the Device trait for IBMDevice.
///
/// Defines standard functions available for roqoqo-iqm devices.
impl Device for IBMDevice {
    /// Returns the gate time of a single qubit operation if the single qubit operation is available on device.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of a single qubit gate.
    /// * `qubit` - The qubit the gate acts on.
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    ///
    fn single_qubit_gate_time(&self, hqslang: &str, qubit: &usize) -> Option<f64> {
        match self {
            IBMDevice::IBMLagosDevice(x) => x.single_qubit_gate_time(hqslang, qubit),
            IBMDevice::IBMNairobiDevice(x) => x.single_qubit_gate_time(hqslang, qubit),
            IBMDevice::IBMOsloDevice(x) => x.single_qubit_gate_time(hqslang, qubit),
            IBMDevice::IBMPerthDevice(x) => x.single_qubit_gate_time(hqslang, qubit),
            IBMDevice::IBMBelemDevice(x) => x.single_qubit_gate_time(hqslang, qubit),
            IBMDevice::IBMJakartaDevice(x) => x.single_qubit_gate_time(hqslang, qubit),
            IBMDevice::IBMLimaDevice(x) => x.single_qubit_gate_time(hqslang, qubit),
            IBMDevice::IBMManilaDevice(x) => x.single_qubit_gate_time(hqslang, qubit),
            IBMDevice::IBMQuitoDevice(x) => x.single_qubit_gate_time(hqslang, qubit),
        }
    }

    /// Returns the gate time of a two qubit operation if the two qubit operation is available on device.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of a two qubit gate.
    /// * `control` - The control qubit the gate acts on.
    /// * `target` - The target qubit the gate acts on.
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    ///
    fn two_qubit_gate_time(&self, hqslang: &str, control: &usize, target: &usize) -> Option<f64> {
        match self {
            IBMDevice::IBMLagosDevice(x) => x.two_qubit_gate_time(hqslang, control, target),
            IBMDevice::IBMNairobiDevice(x) => x.two_qubit_gate_time(hqslang, control, target),
            IBMDevice::IBMOsloDevice(x) => x.two_qubit_gate_time(hqslang, control, target),
            IBMDevice::IBMPerthDevice(x) => x.two_qubit_gate_time(hqslang, control, target),
            IBMDevice::IBMBelemDevice(x) => x.two_qubit_gate_time(hqslang, control, target),
            IBMDevice::IBMJakartaDevice(x) => x.two_qubit_gate_time(hqslang, control, target),
            IBMDevice::IBMLimaDevice(x) => x.two_qubit_gate_time(hqslang, control, target),
            IBMDevice::IBMManilaDevice(x) => x.two_qubit_gate_time(hqslang, control, target),
            IBMDevice::IBMQuitoDevice(x) => x.two_qubit_gate_time(hqslang, control, target),
        }
    }

    /// Returns the gate time of a three qubit operation if the three qubit operation is available on device.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of a two qubit gate.
    /// * `control` - The control qubit the gate acts on.
    /// * `target` - The target qubit the gate acts on.
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    ///
    fn three_qubit_gate_time(
        &self,
        hqslang: &str,
        control_0: &usize,
        control_1: &usize,
        target: &usize,
    ) -> Option<f64> {
        match self {
            IBMDevice::IBMLagosDevice(x) => {
                x.three_qubit_gate_time(hqslang, control_0, control_1, target)
            }
            IBMDevice::IBMNairobiDevice(x) => {
                x.three_qubit_gate_time(hqslang, control_0, control_1, target)
            }
            IBMDevice::IBMOsloDevice(x) => {
                x.three_qubit_gate_time(hqslang, control_0, control_1, target)
            }
            IBMDevice::IBMPerthDevice(x) => {
                x.three_qubit_gate_time(hqslang, control_0, control_1, target)
            }
            IBMDevice::IBMBelemDevice(x) => {
                x.three_qubit_gate_time(hqslang, control_0, control_1, target)
            }
            IBMDevice::IBMJakartaDevice(x) => {
                x.three_qubit_gate_time(hqslang, control_0, control_1, target)
            }
            IBMDevice::IBMLimaDevice(x) => {
                x.three_qubit_gate_time(hqslang, control_0, control_1, target)
            }
            IBMDevice::IBMManilaDevice(x) => {
                x.three_qubit_gate_time(hqslang, control_0, control_1, target)
            }
            IBMDevice::IBMQuitoDevice(x) => {
                x.three_qubit_gate_time(hqslang, control_0, control_1, target)
            }
        }
    }

    /// Returns the gate time of a multi qubit operation if the multi qubit operation is available on device.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of a multi qubit gate.
    /// * `qubits` - The qubits the gate acts on.
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    ///
    fn multi_qubit_gate_time(&self, hqslang: &str, qubits: &[usize]) -> Option<f64> {
        match self {
            IBMDevice::IBMLagosDevice(x) => x.multi_qubit_gate_time(hqslang, qubits),
            IBMDevice::IBMNairobiDevice(x) => x.multi_qubit_gate_time(hqslang, qubits),
            IBMDevice::IBMOsloDevice(x) => x.multi_qubit_gate_time(hqslang, qubits),
            IBMDevice::IBMPerthDevice(x) => x.multi_qubit_gate_time(hqslang, qubits),
            IBMDevice::IBMBelemDevice(x) => x.multi_qubit_gate_time(hqslang, qubits),
            IBMDevice::IBMJakartaDevice(x) => x.multi_qubit_gate_time(hqslang, qubits),
            IBMDevice::IBMLimaDevice(x) => x.multi_qubit_gate_time(hqslang, qubits),
            IBMDevice::IBMManilaDevice(x) => x.multi_qubit_gate_time(hqslang, qubits),
            IBMDevice::IBMQuitoDevice(x) => x.multi_qubit_gate_time(hqslang, qubits),
        }
    }

    /// Returns the matrix of the decoherence rates of the Lindblad equation.
    ///
    /// # Arguments
    ///
    /// * `qubit` - The qubit for which the rate matrix is returned.
    ///
    /// # Returns
    ///
    /// * `Some<Array2<f64>>` - The decoherence rates.
    /// * `None` - The qubit is not part of the device.
    ///
    fn qubit_decoherence_rates(&self, qubit: &usize) -> Option<ndarray::Array2<f64>> {
        match self {
            IBMDevice::IBMLagosDevice(x) => x.qubit_decoherence_rates(qubit),
            IBMDevice::IBMNairobiDevice(x) => x.qubit_decoherence_rates(qubit),
            IBMDevice::IBMOsloDevice(x) => x.qubit_decoherence_rates(qubit),
            IBMDevice::IBMPerthDevice(x) => x.qubit_decoherence_rates(qubit),
            IBMDevice::IBMBelemDevice(x) => x.qubit_decoherence_rates(qubit),
            IBMDevice::IBMJakartaDevice(x) => x.qubit_decoherence_rates(qubit),
            IBMDevice::IBMLimaDevice(x) => x.qubit_decoherence_rates(qubit),
            IBMDevice::IBMManilaDevice(x) => x.qubit_decoherence_rates(qubit),
            IBMDevice::IBMQuitoDevice(x) => x.qubit_decoherence_rates(qubit),
        }
    }

    /// Returns the number of qubits the device supports.
    ///
    /// # Returns
    ///
    /// `usize` - The number of qubits in the device.
    ///
    fn number_qubits(&self) -> usize {
        match self {
            IBMDevice::IBMLagosDevice(x) => x.number_qubits(),
            IBMDevice::IBMNairobiDevice(x) => x.number_qubits(),
            IBMDevice::IBMOsloDevice(x) => x.number_qubits(),
            IBMDevice::IBMPerthDevice(x) => x.number_qubits(),
            IBMDevice::IBMBelemDevice(x) => x.number_qubits(),
            IBMDevice::IBMJakartaDevice(x) => x.number_qubits(),
            IBMDevice::IBMLimaDevice(x) => x.number_qubits(),
            IBMDevice::IBMManilaDevice(x) => x.number_qubits(),
            IBMDevice::IBMQuitoDevice(x) => x.number_qubits(),
        }
    }

    /// Returns the list of pairs of qubits linked with a native two-qubit-gate in the device.
    ///
    /// A pair of qubits is considered linked by a native two-qubit-gate if the device
    /// can implement a two-qubit-gate between the two qubits without decomposing it
    /// into a sequence of gates that involves a third qubit of the device.
    /// The two-qubit-gate also has to form a universal set together with the available
    /// single qubit gates.
    ///
    /// The returned vectors is a simple, graph-library independent, representation of
    /// the undirected connectivity graph of the device.
    /// It can be used to construct the connectivity graph in a graph library of the users
    /// choice from a list of edges and can be used for applications like routing in quantum algorithms.
    ///
    /// # Returns
    ///
    /// A list (Vec) of pairs of qubits linked with a native two-qubit-gate in the device.
    ///
    fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
        match self {
            IBMDevice::IBMLagosDevice(x) => x.two_qubit_edges(),
            IBMDevice::IBMNairobiDevice(x) => x.two_qubit_edges(),
            IBMDevice::IBMOsloDevice(x) => x.two_qubit_edges(),
            IBMDevice::IBMPerthDevice(x) => x.two_qubit_edges(),
            IBMDevice::IBMBelemDevice(x) => x.two_qubit_edges(),
            IBMDevice::IBMJakartaDevice(x) => x.two_qubit_edges(),
            IBMDevice::IBMLimaDevice(x) => x.two_qubit_edges(),
            IBMDevice::IBMManilaDevice(x) => x.two_qubit_edges(),
            IBMDevice::IBMQuitoDevice(x) => x.two_qubit_edges(),
        }
    }

    /// Turns Device into GenericDevice
    ///
    /// Can be used as a generic interface for devices when a boxed dyn trait object cannot be used
    /// (for example when the interface needs to be serialized)
    ///
    fn to_generic_device(&self) -> roqoqo::devices::GenericDevice {
        match self {
            IBMDevice::IBMLagosDevice(x) => x.to_generic_device(),
            IBMDevice::IBMNairobiDevice(x) => x.to_generic_device(),
            IBMDevice::IBMOsloDevice(x) => x.to_generic_device(),
            IBMDevice::IBMPerthDevice(x) => x.to_generic_device(),
            IBMDevice::IBMBelemDevice(x) => x.to_generic_device(),
            IBMDevice::IBMJakartaDevice(x) => x.to_generic_device(),
            IBMDevice::IBMLimaDevice(x) => x.to_generic_device(),
            IBMDevice::IBMManilaDevice(x) => x.to_generic_device(),
            IBMDevice::IBMQuitoDevice(x) => x.to_generic_device(),
        }
    }
}