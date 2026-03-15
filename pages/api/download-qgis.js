import JSZip from 'jszip';
import axios from 'axios';

// GitHubリポジトリの情報（※ユーザー自身のPublicリポジトリを対象）
const GITHUB_OWNER = 'yamamoto-ryuzo';
const GITHUB_REPO = 'ReEarth_Portal';
const GITHUB_BRANCH = 'main'; // または該当するブランチ名
// ダウンロード対象のディレクトリパス
const TARGET_DIR = 'qgis_launcher/download';

/**
 * GitHub API経由で再帰的にディレクトリツリーを取得し、JSZipで圧縮して返す
 */
export default async function handler(req, res) {
  if (req.method !== 'GET') {
    return res.status(405).json({ message: 'Method Not Allowed' });
  }

  try {
    // 1. GitHub APIからリポジトリの全ツリーを取得（recursive=1）
    // （巨大なリポジトリの場合はツリーが大きくなるため注意が必要ですが、指定フォルダ内の抽出には便利です）
    const treeUrl = `https://api.github.com/repos/${GITHUB_OWNER}/${GITHUB_REPO}/git/trees/${GITHUB_BRANCH}?recursive=1`;
    const { data: treeData } = await axios.get(treeUrl, {
      headers: {
        'Accept': 'application/vnd.github.v3+json',
        // 'Authorization': `Bearer ${process.env.GITHUB_TOKEN}` // Privateリポジトリの場合は必須
      }
    });

    // 2. TARGET_DIR（qgis_launcher/download）以下のファイルのみを抽出
    const targetFiles = treeData.tree.filter(item => 
      item.type === 'blob' && item.path.startsWith(`${TARGET_DIR}/`)
    );

    if (targetFiles.length === 0) {
      return res.status(404).json({ message: 'Target folder not found or is empty on GitHub' });
    }

    const zip = new JSZip();

    // 3. 各ファイルのコンテンツを取得し、JSZipに追加
    // ※ 並列でAPIリクエストを飛ばしすぎるとGitHub APIのRate Limitに引っかかるため、
    // 大量ファイルがある場合は Promise.all の同時実行数を制限する工夫が必要です。
    await Promise.all(targetFiles.map(async (file) => {
      // ファイルの生データを取得
      const rawUrl = `https://raw.githubusercontent.com/${GITHUB_OWNER}/${GITHUB_REPO}/${GITHUB_BRANCH}/${file.path}`;
      const response = await axios.get(rawUrl, { responseType: 'arraybuffer' });
      
      // ZIP内でのパスを設定（qgis_launcher/download/ の部分は取り除く）
      const relativePath = file.path.replace(`${TARGET_DIR}/`, '');
      zip.file(relativePath, response.data);
    }));

    // 4. ZIPストリームを生成してレスポンスとして返す
    res.setHeader('Content-Type', 'application/zip');
    res.setHeader('Content-Disposition', 'attachment; filename="qgis_launcher.zip"');

    const zipStream = zip.generateNodeStream({ type: 'nodebuffer', streamFiles: true });
    zipStream.pipe(res);

  } catch (error) {
    console.error('GitHub fetch or ZIP error:', error.message);
    res.status(500).json({ error: 'Failed to create ZIP from GitHub' });
  }
}
