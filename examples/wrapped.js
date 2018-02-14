var rustysecrets = require('rusty-secrets').wrapped;

var threshold    = 7;
var sharesCount  = 10;
var secret       = Buffer.from("Hello, World"); // <Buffer 48 65 6c 6c 6f 2c 20 57 6f 72 6c 64>
var signedShares = false;

rustysecrets.splitSecret(
  threshold,
  sharesCount,
  secret,
  "text/plain",
  signedShares,
  function(err, shares) {
    console.log(shares);
    // > [ '7-1-Chq1wf8Cf7zIeBj27IAOCvYVKepsUfYSwzZgOA',
    // >   '7-2-ChpBizurStE8hFAIdIuouN80zmZQPoC0L897wg',
    // >   '7-3-ChrmRozMWQGb0Gip93nK1jMrk+lEG1nWgJhylA',
    // >   '7-4-Chq0H67kIAvJYcDpWZ35wY12HWMo4Gzl6iBNKw',
    // >   '7-5-ChoT0hmDM9tuNfhI2m+br2FpQOw8xbWHRXdEfQ',
    // >   '7-6-ChrnmN0qBraaybC2QmQ9HUhIp2AAqsMhqY5fhw',
    // >   '7-7-ChpAVWpNFWY9nYgXwZZfc6RX+u8UjxpDBtlW0Q',
    // >   '7-8-ChpDKpl69KI+tv02A7FbMynypmnYQalHfeMh5A',
    // >   '7-9-Chrk5y4d53KZ4sWXgEM5XcXt++bMZHAl0rQosg',
    // >   '7-10-ChoQreq00h9tHo1pGEif7+zMHGrwCwaDPk0zSA' ]

    recover(shares);
  }
);

function recover(shares) {
  var someShares = shares.slice(1, 8);
  rustysecrets.recoverSecret(someShares, signedShares, function(err, recovered) {
    console.log(recovered);
    // { version: 'INITIAL_RELEASE',
    //   secret: <Buffer 48 65 6c 6c 6f 2c 20 57 6f 72 6c 64>,
    //   mimeType: 'text/plain' }
    });
}
