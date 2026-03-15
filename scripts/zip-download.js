const fs = require('fs');
const path = require('path');
const archiver = require('archiver');

const sourceDir = path.join(__dirname, '../qgis_launcher/download');
const publicDir = path.join(__dirname, '../public');
const destFile = path.join(publicDir, 'qgis_launcher.zip');

if (!fs.existsSync(publicDir)) {
  fs.mkdirSync(publicDir, { recursive: true });
}

const output = fs.createWriteStream(destFile);
const archive = archiver('zip', {
  zlib: { level: 9 }
});

output.on('close', function() {
  console.log(`Generated ZIP file at ${destFile} (${archive.pointer()} bytes)`);
});

archive.on('error', function(err) {
  console.error('Failed to create ZIP file:', err);
  process.exit(1);
});

archive.pipe(output);
// Append files from a sub-directory, putting its contents at the root of archive
archive.directory(sourceDir, false);

archive.finalize();