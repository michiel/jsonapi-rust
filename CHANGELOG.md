<a name=""></a>
## 0.6.2 (2019-01-01)

#### Features

*   Add support for filter query param ([664fb91b](https://github.com/michiel/jsonapi-rust/commit/664fb91bf285c9770d180bf40e5ac84a525d4684))

<a name=""></a>
##  0.6.1 (2018-12-30)

#### Features

*   Support sort query parameters ([b6b1def5](https://github.com/michiel/jsonapi-rust/commit/b6b1def55a769ae9fbbf60915e3ae44111d6b348))

<a name=""></a>
##  0.6.0 (2018-02-14)

#### Features

*   Add conversion of object Vec to jsonapi_document ([1bf60a0](https://github.com/michiel/jsonapi-rust/commit/1bf60a0bd98f1027bb8cc42ddb8fc4ee36a61f4c))
*   Support numeric id in JsonApiModel::to_jsonapi_* ([1f98c88](https://github.com/michiel/jsonapi-rust/commit/1f98c884b80f6d02f28df6d58686908c9068a585))


<a name=""></a>
##  0.5.3 (2017-12-20)

#### Features

*   Box the PrimaryData::Single variant ([bf7a767](https://github.com/michiel/jsonapi-rust/commit/bf7a767bdd70c2829acf18e255393661a0d5b7ed))
*   Use and serialize sparse structs ([75b6bac](https://github.com/michiel/jsonapi-rust/commit/75b6bacf8cff34d03dcfa19e1fc5d743578be2dc))
*   model serialization and deserialization working ([d38093e](https://github.com/michiel/jsonapi-rust/commit/d38093e429afbf0f6f7c49e67db0aa89d7c69915))
*   Implement FromStr for JsonApiDocument and Resource ([fb66803](https://github.com/michiel/jsonapi-rust/commit/fb66803252dd7866713ce93741548a45ba2596ab))
*   Update 'serde*' minimal versions and relax version restrictions ([3723938](https://github.com/michiel/jsonapi-rust/commit/3723938dfa9755cebdbaad6ec8a862a6ad7a529c))
*   Use an empty HashMap if attributes is not supplied ([e0d3712](https://github.com/michiel/jsonapi-rust/commit/e0d3712c9b63e8c04d6e2e8c4df6dfc7eddbef11))

#### Bugfixes

*   fix issues with static slice reference on stable ([647f93a](https://github.com/michiel/jsonapi-rust/commit/647f93a0425eff446c10e644ecfc19f957375ecc))


<a name=""></a>
##  0.5.1 (2017-04-13)

#### Bugfixes

*   Not to include data and errors in a same document ([71e65a8](https://github.com/michiel/jsonapi-rust/commit/71e65a8822235e359029c32af51a23bc911fb37d))


<a name=""></a>
## 0.5.0  (2017-04-08)


#### Features

*   Remove superfluous Pagination impl ([9310e369](https://github.com/michiel/jsonapi-rust/commit/9310e3696518b9cdd00f40d91a9e9bac326f4ff2))
*   Add warn logs for setting query defaults ([a2c6c11a](https://github.com/michiel/jsonapi-rust/commit/a2c6c11a770d308f67b8c7bf2c61d4eca9f18301))
*   Add log crate and error logging ([2283cb97](https://github.com/michiel/jsonapi-rust/commit/2283cb97a57c7b124b94c1f58d1fd49e693aaf55))
*   Add denial of unwanted features ([178bb102](https://github.com/michiel/jsonapi-rust/commit/178bb1029eccb24c36a196d7e0f2eb19721e8e48))
*   Add log crate and error logging ([06ea19b1](https://github.com/michiel/jsonapi-rust/commit/06ea19b1244569c3f4d0406fbc136e7a6e0390ac))
*   Remove obsolete attribute_as_x ([76d8fff0](https://github.com/michiel/jsonapi-rust/commit/76d8fff02f0b7281b40f0136fe65517dc3202d44))
*   Add Optional Meta field to Resource ([9f8d2f0b](https://github.com/michiel/jsonapi-rust/commit/9f8d2f0bd9a8985d5fd82fea88a13055bbf7f067))
*   Initial diff/patch functionality ([0ae612d2](https://github.com/michiel/jsonapi-rust/commit/0ae612d2d002fee26f14e4e286bfef3af4a6caaa))
*   Partial Resource diff implementation ([0686a55f](https://github.com/michiel/jsonapi-rust/commit/0686a55fbfbc4086b406339cd4e18604fad64664))
*   Stub Resource patch/diff functions ([158aa7ba](https://github.com/michiel/jsonapi-rust/commit/158aa7ba156249a2967b07a9903a0fced5b50c35))
*   Stub Resource patch/diff functions ([779e30d9](https://github.com/michiel/jsonapi-rust/commit/779e30d98cacc3b309a4219ff320ea02d89f827c))
*   Add Resource from_str and get_attribute ([436df1ac](https://github.com/michiel/jsonapi-rust/commit/436df1ac2b7e907329ba7471856b064abe156001))



<a name=""></a>
##  0.4.0 (2017-03-05)


#### Features

*   Add initial JsonApiModel trait ([7a3a4a23](https://github.com/michiel/jsonapi-rust/commit/7a3a4a2303d649de89b73e348fc8d4c40feaccf5))
*   Resource function get_attribute_as_number ([67e1e661](https://github.com/michiel/jsonapi-rust/commit/67e1e66152ca7d4e8d2a54d5f9aac7f7f9c1b7bf))
*   Add Relationship functions ([b8de4340](https://github.com/michiel/jsonapi-rust/commit/b8de4340485b854d972bd66e92cc100f860d1dd9))
*   Add Resource functions get_relationship and get_attribute_x ([b1342cbf](https://github.com/michiel/jsonapi-rust/commit/b1342cbf3e02b7f834a037f53b180173ca586d7d))



<a name=""></a>
##  0.3.0 (2017-02-28)


#### Features

*   Add queryparams generation with test cases ([4048fe83](https://github.com/michiel/jsonapi-rust/commit/4048fe8355e3cb6d1df11162384ca7cb34a402db))
*   Make all JsonApiError fields optional ([0aab0ede](https://github.com/michiel/jsonapi-rust/commit/0aab0ede8e96845fc3b99899d25cc528cbbed64e))
*   Add doc tests (#6) ([66388c05](https://github.com/michiel/jsonapi-rust/commit/66388c05dabfc08ad1c53ccec1d2a9c202a906a6))



<a name=""></a>
##  0.2.0 (2017-02-23)

#### Features
*   Optional primary data (#5) ([65c54989](https://github.com/michiel/jsonapi-rust/commit/65c54989a93fe7dae46d1747d81d686a5e39f162))
*   Extend document validation (#3) ([7ce19ed5](https://github.com/michiel/jsonapi-rust/commit/7ce19ed5fa404fbdb7690e430ad9b520301021e8))
*   Merge Document and JsonApiResponse (#2) ([6fe0be44](https://github.com/michiel/jsonapi-rust/commit/6fe0be44e81c46db8dbd658f0f4cbb38cc9283d7))



