import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { UploadBtn } from '@/components/UploadBtn';
import { ExcelDisplay } from '@/components/xmlExtractor/ExcelDisplay';
import { useFileProcessing } from '@/hooks/useFileProcessing';
import { useOpenExcelFile } from '@/hooks/useOpenExcelFile';
import { useDocDir } from '@/hooks/useDocDir';
import { Button } from '../ui/button';

export const FileCard = () => {
  const docDir = useDocDir();
  const { excelPath, setExcelPath, processFiles } = useFileProcessing(docDir);
  const { openExcelFile } = useOpenExcelFile(excelPath);

  const handleNewQuery = () => {
    setExcelPath(null);
  }

  return (
    <Card className="w-full max-w-md shadow-lg max-h-max pb-6">
      <CardHeader className='text-left'>
        <CardTitle className='text-2xl'>Extractor Datos de XML</CardTitle>
        <CardDescription className='text-base'>Selecciona la carpeta que contiene todas las facturas que quieras procesar, asegurate que esten en formato XML</CardDescription>
      </CardHeader>
      <CardContent className='flex flex-col gap-6 items-center justify-center p-0'>
        {excelPath ? (
          <>
            <ExcelDisplay openExcelFile={openExcelFile} />
            <Button onClick={handleNewQuery}>Hacer nueva consulta</Button>
          </>

        ) : (
          <UploadBtn processFiles={processFiles} />
        )}


      </CardContent>
    </Card>
  )

}
export default FileCard;