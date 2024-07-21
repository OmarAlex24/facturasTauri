import React, { useState, useEffect } from 'react';
import { invoke } from "@tauri-apps/api/tauri";
import { open as openDialog } from "@tauri-apps/api/dialog";
import excelLogo from './assets/excel.svg';
import { documentDir } from '@tauri-apps/api/path';

const App: React.FC = () => {
  const [status, setStatus] = useState<string>('');
  const [excelPath, setExcelPath] = useState<string | null>(null);
  const [appDataDirPath, setAppDataDirPath] = useState<string | null>(null);

  useEffect(() => {
    const getAppDataDir = async () => {
      const path = await documentDir();
      setAppDataDirPath(path);
    };
    getAppDataDir();
  }, []);

  const handleFolderSelect = async () => {
    if (!appDataDirPath) {
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

      const result = await invoke('process_xml_folder', {
        folderXmlPath: selectedPath as string,  // Asegúrate de que este nombre coincida
        appDataDirPath: appDataDirPath as string  // con el nombre definido en Rust
      });

      console.log("XML procesados");

      const parsedResult = JSON.parse(result as string);

      setStatus(`Archivos procesados con éxito!`);
      setExcelPath(parsedResult.excel_path);
    } catch (error) {
      setStatus(`Error: ${error}`);
      setExcelPath(null);
    }
  };

  const handleOpenFile = async () => {
    if (excelPath) {
      console.log("Abriendo excel: ", excelPath);
      await invoke('abrir_archivo', { ruta: excelPath as string });
    }
  };

  return (
    <div className="bg-gray-100 flex items-center justify-center min-h-screen">
      <div className="text-center">
        <h1 className="text-3xl font-bold mb-4">Factura XML Processor</h1>
        <button
          onClick={handleFolderSelect}
          className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
        >
          Seleccionar Carpeta
        </button>
        <div className="mt-4 text-sm">
          {status}
        </div>
        {excelPath && (
          <div className="mt-4 text-sm text-green-600 flex items-center justify-center">
            <span>Archivo Excel creado: </span>
            <button onClick={handleOpenFile} className="ml-2 flex items-center text-blue-500 hover:underline">
              <img src={excelLogo} alt="Excel Icon" className="w-18 h-18 mr-1" />
            </button>
          </div>
        )}
      </div>
    </div>
  );
};

export default App;