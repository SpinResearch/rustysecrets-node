
var path = require('path');
var fs = require('fs');

var name = process.platform + '_' + process.arch
var fullPath = path.join(path.dirname(__dirname), 'platform', name + '.node');

if (!fs.existsSync(fullPath)) {
  console.error('ERROR: Architecture `' + arch + '` is currently not supported.')
  system.exit(1);
}

module.exports = require(fullPath);

