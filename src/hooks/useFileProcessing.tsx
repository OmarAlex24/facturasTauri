import { useState } from 'react';
import { open as openDialog } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";

export const useFileProcessing = (docDir: string | null) => {
  const [status, setStatus] = useState<string>('');
  const [excelPath, setExcelPath] = useState<string | null>(null);

  const processFiles = async () => {
    if (!docDir) {
      setStatus('Error: No se pudo obtener el directorio de datos de la aplicación.');
      return;
    }

    try {
      const selectedPath = await openDialog({
        directory: true,
        multiple: false,
      });

      if (selectedPath === null) {
        setStatus('No se seleccionó ninguna carpeta.');
        return;
      }

      setStatus('Procesando archivos...');
      console.log("Seleccionado: ", selectedPath);
      console.log("DocDir: ", docDir);

      let result;

      try {
        result = await invoke('main_xml', {
          folderXmlPath: selectedPath as string,
          docDirPath: docDir as string
        });
      } catch (error) {
        console.log("Error: ", error);
        return;
      }


      console.log("XML procesados");

      const parsedResult = JSON.parse(result as string);

      setStatus(`Archivos procesados con éxito!`);
      setExcelPath(parsedResult.excel_path);
    } catch (error) {
      setStatus(`Error: ${error}`);
      setExcelPath(null);
    }
  };

  return { status, excelPath, setExcelPath, processFiles };
}