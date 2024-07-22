import { useEffect, useState } from 'react';
import { documentDir } from '@tauri-apps/api/path';

export const useDocDir = () => {
  const [docDir, setDocDir] = useState<string | null>(null);

  useEffect(() => {
    const getDocDir = async () => {
      const path = await documentDir();
      setDocDir(path);
    };
    getDocDir();
  }, []);

  return docDir
}