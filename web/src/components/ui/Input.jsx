import React from 'react';
import { Search } from 'lucide-react';

export function Input({ icon, className = '', ...props }) {
  return (
    <div className="relative group">
      {icon && (
        <div className="absolute left-4 top-1/2 -translate-y-1/2 text-slate-400 group-focus-within:text-blue-400 transition-colors">
          {icon}
        </div>
      )}
      <input
        className={`w-full bg-slate-900/50 border border-slate-700 rounded-xl py-3 ${icon ? 'pl-12' : 'pl-4'} pr-4 text-white placeholder-slate-500 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-all ${className}`}
        {...props}
      />
    </div>
  );
}
