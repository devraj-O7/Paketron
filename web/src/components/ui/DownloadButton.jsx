import React, { useState, useEffect } from 'react';
import { Download, Monitor } from 'lucide-react';
import { Button } from './Button';

export function DownloadButton({ className = '', variant = 'primary' }) {
  const [os, setOs] = useState('Windows');

  useEffect(() => {
    const userAgent = window.navigator.userAgent;
    if (userAgent.indexOf('Win') !== -1) setOs('Windows');
    else if (userAgent.indexOf('Mac') !== -1) setOs('macOS');
    else if (userAgent.indexOf('Linux') !== -1) setOs('Linux');
    else if (userAgent.indexOf('Android') !== -1) setOs('Android');
    else if (userAgent.indexOf('like Mac') !== -1) setOs('iOS');
  }, []);

  const handleDownload = () => {
    // Mock download link
    const link = document.createElement('a');
    link.href = 'https://github.com/paketron/paketron/releases/download/v0.1.0/paketron-setup.exe';
    link.download = 'paketron-setup.exe';
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  };

  return (
    <Button variant={variant} onClick={handleDownload} className={className}>
      <Download className="w-5 h-5" />
      Download for {os}
    </Button>
  );
}
