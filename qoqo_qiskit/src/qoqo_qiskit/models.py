# Copyright © 2024 HQS Quantum Simulations GmbH.
#
# Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
# in compliance with the License. You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software distributed under the License
# is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
# or implied. See the License for the specific language governing permissions and limitations under
# the License.
"""Qoqo-qiskit dataclasses."""

from typing import Dict, List
from dataclasses import dataclass, field
# useful: from dataclass import astuple


@dataclass
class Registers:
    """Registers.

    The registers are used to store classical information during the execution of a
    roqoqo circuit and to provide a unified output interface for the different backends.

    Defined by three dictionaries, representing bit, float and complex registers.
    """

    bit_register_dict: Dict[str, List[List[bool]]] = field(default_factory=dict)
    float_register_dict: Dict[str, List[List[float]]] = field(default_factory=dict)
    complex_register_dict: Dict[str, List[List[complex]]] = field(default_factory=dict)


@dataclass
class RegistersWithLengths:
    """Registers, with classical registers lengths."""

    clas_regs_lengths: Dict[str, int] = field(default_factory=dict)
    registers: Registers
