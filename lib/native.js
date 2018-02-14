
var path = require('path');
var fs = require('fs');

var ifElectron = process.versions['electron'] ? '_electron' : '';
var name = [process.platform, '_', process.arch, ifElectron].join('');
var fullPath = path.join(path.dirname(__dirname), 'platform', name + '.node');

if (!fs.existsSync(fullPath)) {
  console.error('ERROR: Architecture `' + arch + '` is currently not supported.')
  system.exit(1);
}

module.exports = require(fullPath);

