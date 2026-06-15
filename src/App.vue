<template>
  <div class="command-center">
    <h2>🛡️ 灵魂成长导师 - 战术数据终端</h2>
    
    <div class="input-group">
      <label for="dataInput">📝 情报内容：</label>
      <textarea 
        id="dataInput" 
        v-model="intelContent" 
        rows="5" 
        placeholder="在此输入需要持久化保存的数据..."
      ></textarea>
    </div>

    <div class="action-bar">
      <button @click="handleSave" :disabled="isLoading" class="btn btn-save">
        💾 保存情报 (Save)
      </button>
      <button @click="handleLoad" :disabled="isLoading" class="btn btn-load">
        📥 读取情报 (Load)
      </button>
    </div>

    <div class="console-log">
      <strong>📡 终端日志：</strong>
      <pre :class="{ 'error': isError }">{{ logMessage }}</pre>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { tauriApi } from './tauri-api.js'; // 确保路径正确

// 响应式状态
const intelContent = ref('【绝密】基石行动成功！动态路径引擎已上线！');
const logMessage = ref('系统初始化完毕，等待指令...');
const isLoading = ref(false);
const isError = ref(false);

const TARGET_FILE = 'combat_report.txt';

// 日志打印函数
const log = (msg, error = false) => {
  const time = new Date().toLocaleTimeString();
  logMessage.value = `[${time}] ${msg}`;
  isError.value = error;
};

// 保存情报
const handleSave = async () => {
  isLoading.value = true;
  log('正在加密传输并写入物理硬盘...');
  try {
    const path = await tauriApi.saveFile(TARGET_FILE, intelContent.value);
    log(`✅ 写入成功！物理路径: ${path}`);
  } catch (err) {
    log(`❌ 写入失败: ${err}`, true);
  } finally {
    isLoading.value = false;
  }
};

// 读取情报
const handleLoad = async () => {
  isLoading.value = true;
  log('正在从物理硬盘读取情报...');
  try {
    const data = await tauriApi.loadFile(TARGET_FILE);
    intelContent.value = data; // 回填数据
    log(`✅ 读取成功！已加载 ${data.length} 个字符。`);
  } catch (err) {
    log(`❌ 读取失败: ${err}`, true);
  } finally {
    isLoading.value = false;
  }
};

// 组件挂载时检查环境
onMounted(() => {
  if (!tauriApi.isTauri()) {
    log('⚠️ 警告：当前在浏览器环境运行，无法使用本地文件系统功能。请使用 npm run tauri dev 启动。', true);
  }
});
</script>

<style scoped>
.command-center {
  padding: 20px;
  font-family: 'Segoe UI', system-ui, sans-serif;
  max-width: 600px;
  margin: 0 auto;
  color: #333;
}
.input-group {
  margin-bottom: 20px;
}
textarea {
  width: 100%;
  padding: 10px;
  margin-top: 5px;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-size: 14px;
  box-sizing: border-box;
}
.action-bar {
  display: flex;
  gap: 10px;
  margin-bottom: 20px;
}
.btn {
  padding: 10px 20px;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: opacity 0.2s;
}
.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
.btn-save { background: #4CAF50; }
.btn-load { background: #2196F3; }
.console-log {
  background: #f8f9fa;
  padding: 15px;
  border-radius: 5px;
  border: 1px solid #e9ecef;
}
pre {
  color: #2e7d32;
  white-space: pre-wrap;
  margin-top: 10px;
  font-family: 'Consolas', monospace;
}
pre.error {
  color: #d32f2f;
}
</style>