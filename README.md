## neon-class

A class demo of [Neon](https://github.com/neon-bindings/neon).

```js
'use strict';

const rust = require('./native');
const User = rust.User;

const user = new User({
  id: 1,
  first_name: 'zhang',
  last_name: 'san'
});

console.log(user.get_id());
console.log(user.get_full_name());

```

print

```
1
zhangsan
```