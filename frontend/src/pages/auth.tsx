import { CheckCircle, Zap } from 'lucide-react';
import { useEffect } from 'react';

const AuthRedirect = () => {
  useEffect(() => {
    // Get URL parameters from Google OAuth redirect using browser API
    const urlParams = new URLSearchParams(window.location.search);
    const code = urlParams.get('code');
    const state = urlParams.get('state');
    const error = urlParams.get('error');
    
    console.log('Auth redirect - Code:', code, 'State:', state, 'Error:', error);
    
    // Build the deep link URL
    let deepLinkUrl = 'pally-app://auth';
    if (code || state || error) {
      const params = new URLSearchParams();
      if (code) params.append('code', code);
      if (state) params.append('state', state);
      if (error) params.append('error', error);
      deepLinkUrl += '?' + params.toString();
    }
    
    console.log('Redirecting to:', deepLinkUrl);
    
    // Add a small delay to ensure the component renders first, then redirect
    setTimeout(() => {
      window.location.href = deepLinkUrl;
    }, 100);
  }, []);

  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-900 via-blue-900 to-gray-900 flex items-center justify-center p-4">
      <div className="text-center max-w-md mx-auto">
        {/* Success Icon */}
        <div className="mb-8">
          <div className="w-24 h-24 bg-green-500/20 rounded-full flex items-center justify-center mx-auto mb-6">
            <CheckCircle className="w-12 h-12 text-green-400" />
          </div>
          <h1 className="text-4xl font-bold text-white mb-2">Authenticated!</h1>
          <p className="text-blue-300 text-base">Redirecting to app...</p>
        </div>
      </div>
    </div>
  );
};

export default AuthRedirect;