import { CheckCircle, Zap } from 'lucide-react';
import { useRouter } from 'next/router';
import { useEffect } from 'react';

const AuthRedirect = () => {
  const router = useRouter();

  useEffect(() => {
    // Get URL parameters from Google OAuth redirect
    const { code, state, error } = router.query;
    
    // Build the deep link URL
    let deepLinkUrl = 'pally-app://auth';
    if (code || state || error) {
      const params = new URLSearchParams();
      if (code) params.append('code', code as string);
      if (state) params.append('state', state as string);
      if (error) params.append('error', error as string);
      deepLinkUrl += '?' + params.toString();
    }

    // Redirect immediately
    window.location.href = deepLinkUrl;
  }, [router.query]);

  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-900 via-blue-900 to-gray-900 flex items-center justify-center p-4">
      <div className="text-center max-w-md mx-auto">
        {/* Success Icon */}
        <div className="mb-8">
          <div className="w-24 h-24 bg-green-500/20 rounded-full flex items-center justify-center mx-auto mb-6">
            <CheckCircle className="w-12 h-12 text-green-400" />
          </div>
          <h1 className="text-4xl font-bold text-white mb-2">Authenticated!</h1>
          <p className="text-blue-300 text-lg">Welcome to Pally App</p>
        </div>

        {/* Redirect Info */}
        <div className="bg-gray-800/50 backdrop-blur-sm rounded-2xl p-6 border border-gray-700/50">
          <div className="flex items-center justify-center gap-3 mb-4">
            <Zap className="w-5 h-5 text-blue-400" />
            <span className="text-gray-300 font-medium">Opening app...</span>
          </div>
          
          <p className="text-gray-400 text-sm">
            Redirecting to Pally App...
          </p>
        </div>

        {/* App Info */}
        <div className="mt-6 text-center">
          <div className="inline-flex items-center gap-2 px-4 py-2 bg-gray-800/30 rounded-full border border-gray-700/50">
            <div className="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
            <span className="text-gray-300 text-sm">OAuth Successful</span>
          </div>
        </div>
      </div>
    </div>
  );
};

export default AuthRedirect;