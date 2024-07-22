import { invoke } from '@tauri-apps/api/tauri';

export const useOpenExcelFile = (excelPath: string | null) => {
  const openExcelFile = async (): Promise<void> => {
    console.log("Intentando abrir excel");
    if (excelPath) {
      console.log("Abriendo excel: ", excelPath);
      await invoke('open_file', { ruta: excelPath as string });
    }
  }

  return { openExcelFile }
}
