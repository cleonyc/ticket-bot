# A simple ticket bot written in rust

Designed to work in 1 guild at a time 


Default prefix is `.`

# Usage
```
.panel <Category ID to open tickets in> <Channel ID to send message in> "<Embed title>" "<Embed description>" "<Interaction Message>"
```
This creates a panel that users can click a button on to open a ticket.

Inside a ticket:
```
.close:  closes ticket and saves transcript
  ╰─➤ .delete: deletes ticket
  ╰─➤ .open: reopens ticket
```

# Running

Requirments:
 - cargo/rust (tested on 1.54 and 1.56-nightly)

I could publish a binary for it but that takes a lot of effort.  
Because this is compiling from source it might take quite a while.
```
git clone
```

# Plans
 - Split `database.rs` into `database/panel.rs` and `database/ticket.rs`
    - Maybe add wrapper around both as the struct Database
 - Add unit testing wherever possible because trying to debug live is never fun, and is more likely to produce production bugs

# Donations

I don't expect any money out of this and I legit have no clue why im putting this here but: 

Crypto would be best, you can email me at `henry0w@henry0w.com` for paypal or other fiat donations, being a minor is tricky.  

```
ETH: 0x70eecE9ad093f8d5402EFf12BaCc35f954bcE27E
BTC: bc1q57q5tmrurzyhfv8jmsmep5rzvekewwkvsuhjw0
```

# Copyright

Copyright (C) 2021  Henry0w `<henry0w at henry0w dot com>`

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
