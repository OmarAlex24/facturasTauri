import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { useFileProcessing } from './hooks/useFileProcessing';
import { useOpenExcelFile } from './hooks/useOpenExcelFile';
import { FileProcessor } from './components/FileProcessor';
import { ExcelDisplay } from './components/ExcelDisplay';
import { useDocDir } from './hooks/useDocDir';

const App: React.FC = () => {
  const docDir = useDocDir();
  const { excelPath, processFiles } = useFileProcessing(docDir);
  const { openExcelFile } = useOpenExcelFile(excelPath);

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-100 to-green-100 flex items-center justify-center p-4">
      <Card className="w-full max-w-md shadow-lg">
        <CardHeader className="text-center">
          <CardTitle className="text-2xl font-bold text-blue-700">Factura XML Processor</CardTitle>
        </CardHeader>
        <CardContent>
          <FileProcessor processFiles={processFiles} />

          {excelPath && (
            <ExcelDisplay openExcelFile={openExcelFile} />
          )}
        </CardContent>
      </Card>
    </div>

  );
};

export default App;