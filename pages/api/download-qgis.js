import fs from 'fs';
import path from 'path';
import archiver from 'archiver';

export default function handler(req, res) {
  if (req.method !== 'GET') {
    return res.status(405).json({ message: 'Method Not Allowed' });
  }

  try {
    const targetDir = path.join(process.cwd(), 'qgis_launcher', 'download');

    if (!fs.existsSync(targetDir)) {
      return res.status(404).json({ message: 'Target folder not found on server' });
    }

    res.setHeader('Content-Type', 'application/zip');
    res.setHeader('Content-Disposition', 'attachment; filename="qgis_launcher.zip"');

    const archive = archiver('zip', {
      zlib: { level: 9 } // Sets the compression level.
    });

    archive.on('error', function(err) {
      console.error('Archive error:', err);
      if (!res.headersSent) {
        res.status(500).json({ error: err.message });
      }
    });

    // Pipe archive data to the response
    archive.pipe(res);

    // append files from a sub-directory, putting its contents at the root of archive
    archive.directory(targetDir, false);

    // finalize the archive
    archive.finalize();

  } catch (error) {
    console.error('ZIP error:', error.message);
    if (!res.headersSent) {
      res.status(500).json({ error: 'Failed to create ZIP' });
    }
  }
}
