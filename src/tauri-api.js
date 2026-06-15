/**
 * Tauri IPC 通讯兵模块
 * 负责屏蔽底层 window.__TAURI__ 细节，提供干净的 API 给 UI 层
 */

// 获取 Tauri v2 的 invoke 方法
const invoke = window.__TAURI__?.core?.invoke;

export const tauriApi = {
  /**
   * 检测当前是否运行在 Tauri 桌面环境中
   * @returns {boolean}
   */
  isTauri: () => !!invoke,

  /**
   * 保存文本情报到本地硬盘
   * @param {string} filename - 文件名 (例如: 'data.txt')
   * @param {string} content - 要保存的文本内容
   * @returns {Promise<string>} 返回保存成功的物理路径
   */
  saveFile: async (filename, content) => {
    if (!invoke) {
      throw new Error("当前不在 Tauri 桌面环境中，无法调用底层 API！");
    }
    try {
      // 调用 Rust 端的 write_text_file 命令
      const path = await invoke('write_text_file', { filename, content });
      return path;
    } catch (error) {
      console.error('[tauriApi.saveFile] 战术失败:', error);
      throw error; // 继续向上抛出，让 UI 层处理
    }
  },

  /**
   * 从本地硬盘读取文本情报
   * @param {string} filename - 文件名
   * @returns {Promise<string>} 返回读取到的文本内容
   */
  loadFile: async (filename) => {
    if (!invoke) {
      throw new Error("当前不在 Tauri 桌面环境中，无法调用底层 API！");
    }
    try {
      // 调用 Rust 端的 read_text_file 命令
      const content = await invoke('read_text_file', { filename });
      return content;
    } catch (error) {
      console.error('[tauriApi.loadFile] 战术失败:', error);
      throw error;
    }
  }
};