import initWasm, { compare_images, ImgByte } from './pkg/img_hash_wasm.js';

async function decodeImage(file) {
  const bitmap = await createImageBitmap(file);
  const canvas = document.createElement('canvas');
  canvas.width = bitmap.width;
  canvas.height = bitmap.height;

  const ctx = canvas.getContext('2d');
  ctx.drawImage(bitmap, 0, 0);
  const imageData = ctx.getImageData(0, 0, bitmap.width, bitmap.height);
  return {
    pixels: new Uint8Array(imageData.data.buffer),
    width: bitmap.width,
    height: bitmap.height,
  };
}

export function bindImageCompare(options) {
  const { file1Input, file2Input, button, onResult } = options;

  button.addEventListener('click', async () => {
    const file1 = file1Input.files?.[0];
    const file2 = file2Input.files?.[0];

    if (!file1 || !file2) {
      if (onResult) onResult(null);
      return;
    }

    try {
      const [img1, img2] = await Promise.all([decodeImage(file1), decodeImage(file2)]);
      const img1Bytes = new ImgByte(img1.pixels, img1.width, img1.height);
      const img2Bytes = new ImgByte(img2.pixels, img2.width, img2.height);

      const diff = compare_images(img1Bytes, img2Bytes);
      if (onResult) onResult(diff);
    } catch (err) {
      console.error('img hash error:', err);
      if (onResult) onResult(null);
    }
  });
}

/**
 * 初始化 wasm 模块（建议手动调用一次）
 */
export async function init() {
  await initWasm();
}
