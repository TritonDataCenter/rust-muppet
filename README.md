<!--
    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

<!--
    Copyright (c) 2019, Joyent, Inc.
-->

# rust-muppt

This is an experimental repository for porting [muppet](https://github.com/joyent/muppet) to rust.

Muppet is an HTTP loadbalancer (haproxy) and small daemon that interacts with
ZooKeeper via registrar.  The muppet daemon will update the loadbalancer with
new configuration as hosts come and go from the given service name.
