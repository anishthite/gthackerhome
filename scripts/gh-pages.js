var ghpages = require('gh-pages');

ghpages.publish(
    'public',// <-- replace yourproject with your repo name
    {
        branch: 'gh-pages',
        repo: 'https://github.com/anishthite/gthackerhome.git',
        user: {
            name: 'Anish Thite',
            email: 'anishthite@gmail.com'
        }
    },
    () => {
        console.log('Deploy Complete!')
    }
)


