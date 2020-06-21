import flask, json
from flask import current_app, Response
from flask_cors import CORS, cross_origin




app = flask.Flask(__name__)
app.config["DEBUG"] = True
CORS(app)
app.config['CORS_HEADERS'] = 'Content-Type'
app.logger.addHandler(logging.StreamHandler(sys.stdout))
app.logger.setLevel(logging.ERROR)
gunicorn_logger = logging.getLogger('gunicorn.error')
app.logger.handlers = gunicorn_logger.handlers
app.logger.setLevel(gunicorn_logger.level)
with app.app_context():
    
    #add user
    @app.route('/create_user', methods=['POST'])
    @cross_origin()
    def pdf():
        if flask.request.method == 'POST':
            email, file = get_file()
            return start_backend(file, email)
        return "nl"
    
    @app.route('/pdf', methods=['POST'])
    @cross_origin()
    def pdf():
        if flask.request.method == 'POST':
            email, file = get_file()
            return start_backend(file, email)
        return "nl"


    #add user
    @app.route('/pdf', methods=['POST'])
    @cross_origin()
    def pdf():
        if flask.request.method == 'POST':
            email, file = get_file()
            return start_backend(file, email)
        return "nl"
    



    #test
    @app.route('/', methods=['POST','GET'])
    @cross_origin()
    def home():
        return "#pg"
    
    # @app.route('/summarize', methods=['POST'])
    # def home():
    #     input_json = flask.request.get_json(force=True)
    #     return package_output(generate_summary(input_json['input']))

if __name__ == "__main__":
    app.run(host='192.168.1.10', port=4242)
