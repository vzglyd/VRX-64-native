import { useEffect } from 'react';
import { createRoot } from 'react-dom/client';
import './management.css';
import { ManagementApp } from './ManagementApp';
import { startManagementLegacy } from './legacy';

function Boot() {
  useEffect(() => {
    startManagementLegacy();
  }, []);

  return <ManagementApp />;
}

const root = document.getElementById('management-root');
if (!root) {
  throw new Error('Missing #management-root');
}

createRoot(root).render(<Boot />);
