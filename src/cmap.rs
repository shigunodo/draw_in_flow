#[derive(Clone, Copy, Debug)]
pub struct CMap {
  pub r: [u8; 256],
  pub g: [u8; 256],
  pub b: [u8; 256],
}

impl CMap {
  pub fn new() -> Self {
    let cmap_f64 = CMapF64::new();
    let mut r = [0u8; 256];
    let mut g = [0u8; 256];
    let mut b = [0u8; 256];
    for i in 0..256 {
      r[i] = (cmap_f64.r[i] * 256.0) as u8;
      g[i] = (cmap_f64.g[i] * 256.0) as u8;
      b[i] = (cmap_f64.b[i] * 256.0) as u8;
    };
    Self {r, g, b}
  }
}

struct CMapF64 {
  r: [f64; 256],
  g: [f64; 256],
  b: [f64; 256],
}

// the colormap "coolwarm" from Matplotlib
// K. Moreland. (2009). In Proceedings of the 5th International Symposium on Visual Computing. DOI 10.1007/978-3-642-10520-3_9.
impl CMapF64 {
  const fn new() -> Self {
    Self {
      r: [0.2298057,
      0.23437707945098038,
      0.2389484589019608,
      0.24351983835294116,
      0.24809121780392157,
      0.2526625972549019,
      0.25723397670588233,
      0.26180535615686273,
      0.26638146835294113,
      0.2711042956470588,
      0.27582712294117645,
      0.2805499502352941,
      0.28527277752941177,
      0.2899956048235294,
      0.29471843211764703,
      0.2994412594117647,
      0.3041742870039216,
      0.30906031906666664,
      0.31394635112941177,
      0.31883238319215684,
      0.32371841525490197,
      0.32860444731764704,
      0.33349047938039217,
      0.3383765114431373,
      0.34327752343529416,
      0.34832334141176474,
      0.3533691593882353,
      0.3584149773647059,
      0.3634607953411765,
      0.36850661331764706,
      0.37355243129411764,
      0.3785982492705882,
      0.383662065772549,
      0.38885187195294113,
      0.3940416781333333,
      0.39923148431372546,
      0.4044212904941176,
      0.4096110966745098,
      0.41480090285490195,
      0.4199907090352941,
      0.42519897019607844,
      0.4305068882509804,
      0.4358148063058824,
      0.4411227243607843,
      0.4464306424156863,
      0.4517385604705882,
      0.4570464785254902,
      0.4623543965803922,
      0.46767809468235294,
      0.4730701729882353,
      0.4784622512941176,
      0.48385432959999997,
      0.48924640790588236,
      0.49463848621176465,
      0.500030564517647,
      0.5054226428235293,
      0.5108243242509803,
      0.5162603025411764,
      0.5216962808313725,
      0.5271322591215686,
      0.5325682374117646,
      0.5380042157019607,
      0.5434401939921568,
      0.5488761722823529,
      0.5543118699137254,
      0.5597467255686274,
      0.5651815812235294,
      0.5706164368784313,
      0.5760512925333333,
      0.5814861481882353,
      0.5869210038431373,
      0.5923558594980393,
      0.5977767754941177,
      0.603162067917647,
      0.6085473603411764,
      0.6139326527647059,
      0.6193179451882354,
      0.6247032376117647,
      0.6300885300352941,
      0.6354738224588236,
      0.640827782372549,
      0.6461128107647058,
      0.6513978391568627,
      0.6566828675490196,
      0.6619678959411764,
      0.6672529243333334,
      0.6725379527254902,
      0.677822981117647,
      0.6830556815607843,
      0.6881884831921569,
      0.6933212848235294,
      0.6984540864549019,
      0.7035868880862746,
      0.7087196897176471,
      0.7138524913490196,
      0.7189852929803922,
      0.724041371882353,
      0.7289695795686274,
      0.7338977872549018,
      0.7388259949411764,
      0.743754202627451,
      0.7486824103137254,
      0.753610618,
      0.7585388256862745,
      0.7633627801019607,
      0.7680343643529411,
      0.7727059486039215,
      0.777377532854902,
      0.7820491171058823,
      0.7867207013568628,
      0.7913922856078431,
      0.7960638698588236,
      0.8006008472941177,
      0.8049647588235295,
      0.8093286703529411,
      0.8136925818823529,
      0.8180564934117647,
      0.8224204049411765,
      0.8267843164705883,
      0.831148228,
      0.8353447113529412,
      0.839351442772549,
      0.8433581741921568,
      0.8473649056117647,
      0.8513716370313725,
      0.8553783684509804,
      0.8593850998705882,
      0.863391831290196,
      0.8674276350862745,
      0.8714925112588235,
      0.8755573874313726,
      0.8796222636039216,
      0.8836871397764705,
      0.8877520159490196,
      0.8918168921215687,
      0.8958817682941177,
      0.8995432066000001,
      0.9028486703176472,
      0.9061541340352941,
      0.9094595977529412,
      0.9127650614705882,
      0.9160705251882353,
      0.9193759889058823,
      0.9226814526235294,
      0.925563423,
      0.9281160096666666,
      0.9306685963333333,
      0.933221183,
      0.9357737696666666,
      0.9383263563333333,
      0.940878943,
      0.9434315296666667,
      0.945540298909804,
      0.9473454036,
      0.9491505082901961,
      0.9509556129803922,
      0.9527607176705882,
      0.9545658223607844,
      0.9563709270509804,
      0.9581760317411765,
      0.9595176584705882,
      0.9605811984235294,
      0.9616447383764706,
      0.9627082783294117,
      0.9637718182823529,
      0.9648353582352941,
      0.9658988981882353,
      0.9669624381411764,
      0.9675442976352941,
      0.9678738483176471,
      0.968203399,
      0.9685329496823529,
      0.9688625003647059,
      0.9691920510470589,
      0.9695216017294117,
      0.9698511524117647,
      0.9696829796666666,
      0.9692885689999999,
      0.9688941583333334,
      0.9684997476666667,
      0.968105337,
      0.9677109263333333,
      0.9673165156666667,
      0.966922105,
      0.9660167198392157,
      0.9649113881372549,
      0.963806056435294,
      0.9627007247333333,
      0.9615953930313725,
      0.9604900613294117,
      0.9593847296274509,
      0.9582793979254902,
      0.9566532109764706,
      0.9548534056117647,
      0.9530536002470588,
      0.951253794882353,
      0.9494539895176471,
      0.9476541841529411,
      0.9458543787882353,
      0.9440545734235294,
      0.9417279298235294,
      0.9392537715176471,
      0.9367796132117647,
      0.9343054549058823,
      0.9318312966,
      0.9293571382941177,
      0.9268829799882353,
      0.9244088216823529,
      0.921406221227451,
      0.9182816725843137,
      0.9151571239411764,
      0.9120325752980393,
      0.908908026654902,
      0.9057834780117647,
      0.9026589293686275,
      0.8995343807254902,
      0.8958845948352941,
      0.8921375427882353,
      0.8883904907411765,
      0.8846434386941177,
      0.8808963866470588,
      0.8771493346,
      0.8734022825529412,
      0.8696552305058823,
      0.8653913329372549,
      0.8610536002941176,
      0.8567158676509804,
      0.8523781350078431,
      0.848040402364706,
      0.8437026697215686,
      0.8393649370784314,
      0.8350272044352941,
      0.8301865219490197,
      0.8252938101686275,
      0.8204010983882353,
      0.8155083866078432,
      0.810615674827451,
      0.8057229630470588,
      0.8008302512666666,
      0.7959375394862745,
      0.7905615319411765,
      0.7851533046784314,
      0.7797450774156863,
      0.7743368501529412,
      0.7689286228901963,
      0.763520395627451,
      0.758112168364706,
      0.7527039411019608,
      0.7468380122117647,
      0.7409573187529412,
      0.7350766252941177,
      0.7291959318352941,
      0.7233152383764706,
      0.717434544917647,
      0.7115538514588235,
      0.705673158],
      g: [0.298717966,
      0.3055417303294118,
      0.3123654946588235,
      0.3191892589882353,
      0.3260130233176471,
      0.3328367876470588,
      0.3396605519764706,
      0.3464843163058824,
      0.35330440842352945,
      0.3600106619764706,
      0.36671691552941177,
      0.37342316908235296,
      0.38012942263529415,
      0.3868356761882353,
      0.39354192974117647,
      0.40024818329411765,
      0.40694488283921565,
      0.41349827226666663,
      0.4200516616941176,
      0.4266050511215686,
      0.4331584405490196,
      0.43971182997647057,
      0.44626521940392155,
      0.45281860883137254,
      0.45935363472941176,
      0.4657111465098039,
      0.47206865829019606,
      0.47842617007058824,
      0.4847836818509804,
      0.49114119363137254,
      0.4974987054117647,
      0.5038562171921569,
      0.5101834172862746,
      0.5162984355764706,
      0.5224134538666667,
      0.5285284721568628,
      0.5346434904470588,
      0.540758508737255,
      0.5468735270274511,
      0.5529885453176471,
      0.559058179764706,
      0.5648827414588236,
      0.5707073031529412,
      0.5765318648470589,
      0.5823564265411765,
      0.5881809882352941,
      0.5940055499294118,
      0.5998301116235294,
      0.6055912316235293,
      0.6110774376156862,
      0.6165636436078431,
      0.6220498496,
      0.6275360555921569,
      0.6330222615843136,
      0.6385084675764706,
      0.6439946735686275,
      0.6493966148235294,
      0.6544976105882353,
      0.6595986063529412,
      0.664699602117647,
      0.6698005978823529,
      0.6749015936470587,
      0.6800025894117647,
      0.6851035851764705,
      0.6900970112156862,
      0.6947677280784313,
      0.6994384449411764,
      0.7041091618039216,
      0.7087798786666667,
      0.7134505955294117,
      0.7181213123921568,
      0.7227920292549019,
      0.7273297248823529,
      0.7315274773529412,
      0.7357252298235294,
      0.7399229822941177,
      0.7441207347647059,
      0.7483184872352941,
      0.7525162397058823,
      0.7567139921764706,
      0.7607515064117647,
      0.7644364965294117,
      0.7681214866470587,
      0.7718064767647058,
      0.7754914668823529,
      0.7791764569999999,
      0.782861447117647,
      0.786546437235294,
      0.790042626890196,
      0.7931783792980391,
      0.7963141317058823,
      0.7994498841137254,
      0.8025856365215686,
      0.8057213889294117,
      0.8088571413372548,
      0.811992893745098,
      0.8149103926470588,
      0.8174641357058824,
      0.8200178787647059,
      0.8225716218235294,
      0.8251253648823529,
      0.8276791079411765,
      0.830232851,
      0.8327865940588235,
      0.8350922218196078,
      0.8370352195294117,
      0.8389782172392156,
      0.8409212149490196,
      0.8428642126588235,
      0.8448072103686275,
      0.8467502080784314,
      0.8486932057882353,
      0.8503583215607843,
      0.8516661605568627,
      0.8529739995529412,
      0.8542818385490196,
      0.8555896775450981,
      0.8568975165411765,
      0.8582053555372549,
      0.8595131945333333,
      0.8605139972941176,
      0.861166825654902,
      0.8618196540156863,
      0.8624724823764706,
      0.8631253107372548,
      0.8637781390980391,
      0.8644309674588235,
      0.8650837958196078,
      0.864376599772549,
      0.8623093793176471,
      0.860242158862745,
      0.8581749384078431,
      0.8561077179529412,
      0.8540404974980391,
      0.8519732770431372,
      0.8499060565882353,
      0.8475002359999999,
      0.8447956505882352,
      0.8420910651764706,
      0.8393864797647058,
      0.8366818943529412,
      0.8339773089411765,
      0.8312727235294118,
      0.8285681381176471,
      0.8255172980705883,
      0.8221971488627451,
      0.818876999654902,
      0.8155568504470588,
      0.8122367012392158,
      0.8089165520313726,
      0.8055964028235294,
      0.8022762536156862,
      0.7986057405333333,
      0.7946955048,
      0.7907852690666667,
      0.7868750333333333,
      0.7829647976,
      0.7790545618666667,
      0.7751443261333334,
      0.7712340904,
      0.7669728545098039,
      0.7625010185254902,
      0.7580291825411765,
      0.7535573465568628,
      0.7490855105725491,
      0.7446136745882352,
      0.7401418386039216,
      0.7356700026196078,
      0.7308497161882352,
      0.7258469080941177,
      0.7208441,
      0.7158412919058823,
      0.7108384838117647,
      0.705835675717647,
      0.7008328676235294,
      0.6958300595294117,
      0.6904839307372549,
      0.6849817470823529,
      0.679479563427451,
      0.673977379772549,
      0.6684751961176472,
      0.6629730124627451,
      0.6574708288078431,
      0.6519686451529412,
      0.6461297415882352,
      0.6401590780588234,
      0.6341884145294118,
      0.628217751,
      0.6222470874705882,
      0.6162764239411764,
      0.6103057604117648,
      0.604335096882353,
      0.598033822717647,
      0.5916223450078432,
      0.5852108672980392,
      0.5787993895882353,
      0.5723879118784315,
      0.5659764341686274,
      0.5595649564588235,
      0.5531534787490197,
      0.5464134770196079,
      0.5395814885647059,
      0.5327495001098039,
      0.525917511654902,
      0.5190855232,
      0.512253534745098,
      0.5054215462901961,
      0.49858955783529413,
      0.49142041718431373,
      0.48417347218039214,
      0.4769265271764706,
      0.469679582172549,
      0.46243263716862765,
      0.4551856921647059,
      0.4479387471607843,
      0.4406918021568627,
      0.43307455670588235,
      0.4253887370980392,
      0.41770291749019606,
      0.41001709788235297,
      0.4023312782745098,
      0.39464545866666667,
      0.3869596390588235,
      0.37927381945098043,
      0.3711276720470588,
      0.3629157635294118,
      0.3547038550117647,
      0.34649194649411763,
      0.3382800379764708,
      0.3300681294588235,
      0.32185622094117644,
      0.3136443124235294,
      0.30473276355294115,
      0.2957488380941176,
      0.2867649126352941,
      0.2777809871764706,
      0.26879706171764706,
      0.2598131362588235,
      0.2508292108,
      0.24184528534117647,
      0.23139699905882352,
      0.2208510887215686,
      0.21030517838431373,
      0.19975926804705882,
      0.18921335770980421,
      0.17866744737254903,
      0.1681215370352941,
      0.1575756266980392,
      0.14002101948235293,
      0.12224032527058823,
      0.10445963105882351,
      0.08667893684705881,
      0.06889824263529411,
      0.05111754842352939,
      0.03333685421176469,
      0.01555616],
      b: [0.753683153,
      0.7596795275882353,
      0.7656759021764705,
      0.7716722767647058,
      0.7776686513529412,
      0.7836650259411765,
      0.7896614005294117,
      0.795657775117647,
      0.8016373194980392,
      0.8070951274352941,
      0.812552935372549,
      0.818010743309804,
      0.8234685512470589,
      0.8289263591843137,
      0.8343841671215686,
      0.8398419750588235,
      0.8452627266980393,
      0.8501276338666667,
      0.8549925410352941,
      0.8598574482039216,
      0.864722355372549,
      0.8695872625411765,
      0.8744521697098039,
      0.8793170768784313,
      0.8841219216235294,
      0.8883461629411764,
      0.8925704042588235,
      0.8967946455764706,
      0.9010188868941177,
      0.9052431282117647,
      0.9094673695294118,
      0.9136916108470589,
      0.9178306732313726,
      0.9213734830823529,
      0.9249162929333333,
      0.9284591027843138,
      0.9320019126352941,
      0.9355447224862745,
      0.939087532337255,
      0.9426303421882353,
      0.9460614570784314,
      0.9488894191803922,
      0.951717381282353,
      0.9545453433843137,
      0.9573733054862745,
      0.9602012675882352,
      0.963029229690196,
      0.9658571917921568,
      0.9685462810941176,
      0.970633588262745,
      0.9727208954313725,
      0.9748082026,
      0.9768955097686275,
      0.9789828169372549,
      0.9810701241058823,
      0.9831574312745098,
      0.9850787763764707,
      0.9864073998117647,
      0.9877360232470589,
      0.9890646466823529,
      0.9903932701176471,
      0.9917218935529412,
      0.9930505169882353,
      0.9943791404235295,
      0.9955155482352941,
      0.9960753091764706,
      0.9966350701176471,
      0.9971948310588236,
      0.997754592,
      0.9983143529411764,
      0.9988741138823529,
      0.9994338748235294,
      0.9997767317764705,
      0.9995652785372549,
      0.9993538252980392,
      0.9991423720588235,
      0.9989309188196078,
      0.9987194655803922,
      0.9985080123411765,
      0.9982965591019608,
      0.9978457748823529,
      0.9968684625058823,
      0.9958911501294118,
      0.9949138377529412,
      0.9939365253764706,
      0.992959213,
      0.9919819006235294,
      0.9910045882470588,
      0.9897684281843138,
      0.9880381043568628,
      0.9863077805294118,
      0.9845774567019608,
      0.9828471328745098,
      0.9811168090470588,
      0.9793864852196078,
      0.9776561613921568,
      0.9756509706470589,
      0.973187668372549,
      0.9707243660980392,
      0.9682610638235294,
      0.9657977615490196,
      0.9633344592745098,
      0.960871157,
      0.9584078547254902,
      0.9556576765568627,
      0.9524882182352941,
      0.9493187599137255,
      0.9461493015921568,
      0.9429798432705883,
      0.9398103849490196,
      0.9366409266274509,
      0.9334714683058823,
      0.9300075603921568,
      0.9261650744313725,
      0.9223225884705882,
      0.9184801025098039,
      0.9146376165490196,
      0.9107951305882354,
      0.906952644627451,
      0.9031101586666667,
      0.8989704099411765,
      0.8944937634156863,
      0.8900171168901961,
      0.885540470364706,
      0.8810638238392158,
      0.8765871773137255,
      0.8721105307882353,
      0.8676338842627451,
      0.8626024620196079,
      0.8570162640588236,
      0.8514300660980393,
      0.845843868137255,
      0.8402576701764708,
      0.8346714722156863,
      0.829085274254902,
      0.8234990762941177,
      0.8177890744,
      0.8119698337411765,
      0.8061505930823529,
      0.8003313524235294,
      0.7945121117647058,
      0.7886928711058824,
      0.7828736304470588,
      0.7770543897882353,
      0.7711363078117647,
      0.765141349254902,
      0.7591463906980392,
      0.7531514321411764,
      0.7471564735843139,
      0.741161515027451,
      0.7351665564705883,
      0.7291715979137255,
      0.7231054172980392,
      0.7169905058,
      0.7108755943019608,
      0.7047606828039216,
      0.6986457713058823,
      0.6925308598078431,
      0.6864159483098039,
      0.6803010368117647,
      0.6741447150392157,
      0.6679635471019607,
      0.6617823791647058,
      0.655601211227451,
      0.6494200432901962,
      0.6432388753529412,
      0.6370577074156862,
      0.6308765394784314,
      0.6246854782352941,
      0.6184892347843137,
      0.6122929913333334,
      0.6060967478823529,
      0.5999005044313725,
      0.5937042609803921,
      0.5875080175294117,
      0.5813117740784314,
      0.5751383613647059,
      0.5689753262588235,
      0.5628122911529412,
      0.5566492560470588,
      0.5504862209411766,
      0.5443231858352942,
      0.5381601507294118,
      0.5319971156235295,
      0.5258903482588235,
      0.5198055987058824,
      0.5137208491529413,
      0.5076360996,
      0.5015513500470589,
      0.49546660049411767,
      0.4893818509411765,
      0.48329710138823534,
      0.4773022923529412,
      0.4713374634901961,
      0.465372634627451,
      0.4594078057647059,
      0.453442976901961,
      0.4474781480392157,
      0.44151331917647063,
      0.4355484903137255,
      0.429707070372549,
      0.4239002049294118,
      0.41809333948627453,
      0.4122864740431373,
      0.4064796086,
      0.40067274315686274,
      0.3948658777137255,
      0.38905901227058826,
      0.38340843537647057,
      0.37779392507058823,
      0.3721794147647059,
      0.36656490445882356,
      0.36095039415294133,
      0.35533588384705883,
      0.3497213735411765,
      0.34410686323529416,
      0.3386806345176471,
      0.33328927276078435,
      0.32789791100392157,
      0.32250654924705885,
      0.3171151874901961,
      0.31172382573333335,
      0.3063324639764706,
      0.30094110221960785,
      0.2957689564156863,
      0.2906281271764706,
      0.2854872979372549,
      0.2803464686980392,
      0.2752056394588237,
      0.2700648102196078,
      0.26492398098039216,
      0.25978315174117644,
      0.25489142806666665,
      0.2500254739333333,
      0.2451595198,
      0.24029356566666665,
      0.23542761153333333,
      0.2305616574,
      0.22569570326666666,
      0.22082974913333334,
      0.21624203829411764,
      0.21167287700784312,
      0.20710371572156863,
      0.2025345544352941,
      0.19796539314901973,
      0.1933962318627451,
      0.18882707057647058,
      0.1842579092901961,
      0.17999609695686275,
      0.17574419910588235,
      0.17149230125490195,
      0.16724040340392157,
      0.16298850555294117,
      0.15873660770196077,
      0.1544847098509804,
      0.150232812],
    }
  }
}