import { Button } from "@/components/ui/button"
import uploadIcon from '@/assets/upload.svg';

interface FileProcessorProps {
  processFiles: () => void;
}

export const UploadBtn = ({ processFiles }: FileProcessorProps) => {
  return (
    <>
      <Button onClick={processFiles} className="gap-4">
        <img src={uploadIcon} alt="upload icon" />
        Seleccionar Carpeta
      </Button >
    </>
  )
}