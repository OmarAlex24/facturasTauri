import { Card, CardContent } from '@/components/ui/card';
import excelLogo from '@/assets/excel.svg';

interface ExcelDisplayProps {
  openExcelFile: () => Promise<void>;
}

export const ExcelDisplay = ({ openExcelFile }: ExcelDisplayProps) => {
  return (
    <>
      <Card className="bg-green-50 border-green-200">
        <CardContent className="text-center py-4">
          <p className="text-green-700 font-medium mb-2">Archivo Excel creado:</p>
          <button onClick={openExcelFile} >
            <img src={excelLogo} alt="Excel Icon" className="w-18 h-18 mr-1" />
          </button>
        </CardContent>
      </Card>
    </>
  )

}