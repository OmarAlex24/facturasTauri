import React, { useState, useEffect } from 'react';
import { invoke } from "@tauri-apps/api/tauri";
import { open as openDialog } from "@tauri-apps/api/dialog";
import excelLogo from './assets/excel.svg';
import uploadIcon from './assets/upload.svg'
import { documentDir } from '@tauri-apps/api/path';
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';


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

      const result = await invoke('main_xml', {
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
    // <>
    //   <div className="bg-gray-100 flex flex-col p-14 min-h-screen">
    //     <h1 className="text-3xl font-bold mb-4">Factura XML Processor</h1>
    //     <div className="text-center">
    //       <button
    //         onClick={handleFolderSelect}
    //         className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
    //       >
    //         Seleccionar Carpeta
    //       </button>
    //       <div className="mt-4 text-sm">
    //         {status}
    //       </div>
    //       {excelPath && (
    //         <div className="mt-4 text-sm text-green-600 flex flex-col items-center justify-center">
    //           <span>Archivo Excel creado: </span>
    //           <Button onClick={handleOpenFile} >
    //             <img src={excelLogo} alt="Excel Icon" className="w-18 h-18 mr-1" />
    //           </Button>
    //         </div>
    //       )}
    //     </div>
    //   </div>
    // </>

    <div className="min-h-screen bg-gradient-to-br from-blue-100 to-green-100 flex items-center justify-center p-4">
      <Card className="w-full max-w-md shadow-lg">
        <CardHeader className="text-center">
          <CardTitle className="text-2xl font-bold text-blue-700">Factura XML Processor</CardTitle>
        </CardHeader>
        <CardContent>
          <Button onClick={handleFolderSelect}
            className="w-full mb-6 flex gap-3 bg-blue-500 hover:bg-blue-600 text-white">
            <img src={uploadIcon} alt="upload icon" />
            Seleccionar Carpeta
          </Button>

          <div className="text-center mb-6">
            <p className="text-green-600 font-semibold mb-2">
              {status}
            </p>
          </div>

          {excelPath && (
            <Card className="bg-green-50 border-green-200">
              <CardContent className="text-center py-4">
                <p className="text-green-700 font-medium mb-2">Archivo Excel creado:</p>
                <button onClick={handleOpenFile} >
                  <img src={excelLogo} alt="Excel Icon" className="w-18 h-18 mr-1" />
                </button>
              </CardContent>
            </Card>
          )}
        </CardContent>
      </Card>
    </div>

  );
};

export default App;