var searchIndex = {};
searchIndex["s3lsio"] = {"doc":"If you want a configuration file to store options so that you don&#39;t want to pass those in\neach time then create a subdirectory in your home directory:\n```mkdir ~/.s3lsio```\nCreate a TOML file called config:\n```vim ~/.s3lsio/config```\nAdd the following options (optional):\n[options]\nendpoint = &quot;&lt;whatever endpoint you want&gt;&quot;\nproxy = &quot;&lt;whatever your proxy url with port if you use a proxy&gt;&quot;\nsignature = &quot;V4&quot;","items":[[3,"Error","s3lsio","Allows you to control Error output.",null,null],[12,"format","","Defaults to OutputFormat::serialize since it&#39;s easier to debug.",0,null],[12,"color","","Can be any term color. Defaults to term::color::RED.",0,null],[3,"Output","","Allows you to control non-Error output.",null,null],[12,"format","","Defaults to OutputFormat::plain.",1,null],[12,"color","","Can be any term color. Defaults to term::color::GREEN.",1,null],[3,"Client","","Client structure holds a reference to the ```S3Client``` which also implements two traits:\n```AwsCredentialsProvider``` and ```DispatchSignedRequest```\nSince ```S3Client``` struct is takes those two traits as parameters then ALL functions called\nthat require passing in ```S3Client``` or Client must specify the trait signature as follows:\nExample: fn ```whatever_function```&lt;P: ```AwsCredentialsProvider```, D: ```DispatchSignedRequest```&gt;(client: &amp;mut Client&lt;P,D&gt;)\nNote: Could also specify &#39;where&#39; P:... D:... instead.",null,null],[12,"s3client","","",2,null],[12,"config","","",2,null],[12,"error","","",2,null],[12,"output","","",2,null],[12,"is_quiet","","",2,null],[12,"is_time","","",2,null],[12,"is_bench","","",2,null],[12,"is_compute_hash","","",2,null],[4,"OutputFormat","","Allows you to set the output type for stderr and stdout.",null,null],[13,"CSV","","",3,null],[13,"JSON","","",3,null],[13,"PrettyJSON","","",3,null],[13,"Plain","","",3,null],[13,"Serialize","","",3,null],[13,"Simple","","",3,null],[13,"None","","",3,null],[13,"NoneAll","","",3,null],[4,"Commands","","Commands",null,null],[13,"abort","","",4,null],[13,"acl","","",4,null],[13,"admin","","",4,null],[13,"cp","","",4,null],[13,"get","","",4,null],[13,"head","","",4,null],[13,"mb","","",4,null],[13,"put","","",4,null],[13,"range","","",4,null],[13,"rb","","",4,null],[13,"rm","","",4,null],[13,"ls","","",4,null],[13,"ver","","",4,null],[11,"fmt","","",3,null],[11,"clone","","",3,null],[11,"eq","","",3,null],[11,"fmt","","",4,null],[11,"clone","","",4,null],[11,"eq","","",4,null],[11,"fmt","","",0,null],[11,"clone","","",0,null],[11,"eq","","",0,null],[11,"ne","","",0,null],[11,"fmt","","",1,null],[11,"clone","","",1,null],[11,"eq","","",1,null],[11,"ne","","",1,null]],"paths":[[3,"Error"],[3,"Output"],[3,"Client"],[4,"OutputFormat"],[4,"Commands"]]};
initSearch(searchIndex);
