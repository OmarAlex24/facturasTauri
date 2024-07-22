import { Button } from "@/components/ui/button"
import uploadIcon from '@/assets/upload.svg';

interface FileProcessorProps {
  processFiles: () => void;
}

export const FileProcessor = ({ processFiles }: FileProcessorProps) => {
  return (
    <>
      <Button onClick={processFiles}
        className="w-full mb-6 flex gap-3 bg-blue-500 hover:bg-blue-600 text-white">
        <img src={uploadIcon} alt="upload icon" />
        Seleccionar Carpeta
      </Button>

      <div className="text-center mb-6">
        <p className="text-green-600 font-semibold mb-2">
          {status}
        </p>
      </div>
    </>
  )
}