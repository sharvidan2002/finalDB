interface HeaderProps {
  className?: string;
}

export function Header({ className }: HeaderProps) {
  return (
    <header className={`bg-gradient-to-r from-slate-800 to-slate-900 text-white shadow-lg ${className}`}>
      <div className="container mx-auto px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-4">
            <div className="flex items-center space-x-2">

            </div>
            <div>
              <h1 className="text-xl font-bold tracking-tight">
                Divisional Forest Office
              </h1>
              <p className="text-sm text-slate-300">
                Vavuniya, Sri Lanka - Staff Management System
              </p>
            </div>
          </div>
          <div className="text-right">
            <p className="text-sm text-slate-300">
              {new Date().toLocaleDateString('en-GB', {
                weekday: 'long',
                year: 'numeric',
                month: 'long',
                day: 'numeric'
              })}
            </p>
            <p className="text-xs text-slate-400">
              Version 1.0.0
            </p>
          </div>
        </div>
      </div>
    </header>
  );
}