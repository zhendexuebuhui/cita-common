// Copyright Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub trait FromProto<T>: Sized {
    fn from_proto(p_val: T) -> Self;
}

pub trait TryFromProto<T>: Sized {
    type Error;

    fn try_from_proto(p_val: T) -> Result<Self, Self::Error>;
}

pub trait TryIntoProto<T>: Sized {
    type Error;

    fn try_into_proto(self) -> Result<T, Self::Error>;
}
