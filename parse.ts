// parse a json string into an object int typescript with Deno
// deno run --allow-net parse-json.ts

const string =
  '[{"id":2,"course":"ckurs","taskid":519,"timestamp":1682179686249,"content":"#include <stdio.h>\\n\\nint main(int argc, char const *argv[])\\n{\\n    printf(\\"Hello World!\\\\n\\");\\n    return 0;\\n}\\n","resultType":"COMPILE_ERROR","simplified":"{\\"compiler\\":{\\"stdout\\":\\"If you want this task to be accepted, you have to ask nicely (include \\\\\\"please\\\\\\" in your submission)\\",\\"exitCode\\":1}}","details":"{}","score":0},{"id":3,"course":"ckurs","taskid":519,"timestamp":1682179709374,"content":"#include <stdio.h>\\n\\nint main(int argc, char const *argv[])\\n{\\n    printf(\\"Hello World!\\\\n\\"); // please :)\\n    return 0;\\n}\\n","resultType":"SUCCESS","simplified":"{\\"compiler\\":{\\"stdout\\":\\"Your wish has been granted!\\",\\"exitCode\\":0}}","details":"{}","score":1}]';

const json = JSON.parse(string);

console.log(json);
