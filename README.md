# rust-decimal-test
This is just some test code to clarify a problem using bson decimals in this issue: https://github.com/mongodb/bson-rust/issues/282

Running the example should connect to a MongoDB instance running on 127.0.0.1, access the `test` database and write a document to the `test` collection. In this document, there are various kind of fields: a date, a Decimal128 and a String. The Date and the String both have the expected value, while the Decimal128 contains a totally different value than the one I provided (should be 42, is 6.6E-1819).

The most strange part is that, reading it again from Rust, the result I get is correct.
