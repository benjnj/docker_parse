USER root
RUN apt-get install -y libfontconfig1-dev
RUN pip install cmake 
RUN pip install igraph 
RUN pip install mnnpy
RUN pip install bbknn 
RUN pip install leidenalg
RUN pip install louvain
RUN pip install scanpy
RUN pip install scvi-tools
RUN pip install 'jedi==0.17.2'
RUN pip install 'parso==0.7.0.'
RUN pip install 'tensorflow==2.4.0'
RUN pip install sccoda
RUN pip install sccaf
RUN pip install mofapy2
RUN R --no-save -e "update.packages(ask = FALSE)"
RUN R --no-save -e "install.packages('BiocManager')"
RUN R --no-save -e "install.packages('Seurat')"
RUN R --no-save -e "install.packages('kableExtra')"
RUN R --no-save -e "install.packages('DirichletReg')"
RUN R --no-save -e "install.packages('glmnet')"
RUN R --no-save -e "install.packages('ggraph')"
RUN R --no-save -e "install.packages('shinythemes')"
RUN R --no-save -e "install.packages('shinyWidgets')"
RUN R --no-save -e "install.packages('remotes')"
RUN R --no-save -e "install.packages('ggalluvial')"
RUN R --no-save -e "install.packages('lmerTest')"
RUN R --no-save -e "install.packages('UpSetR')"
RUN R --no-save -e "install.packages('ggnewscale')"
RUN R --no-save -e "BiocManager::install('GO.db')"
RUN R --no-save -e "BiocManager::install('impute')"
RUN R --no-save -e "install.packages('WGCNA')"
RUN R --no-save -e "BiocManager::install('safe')"
RUN R --no-save -e "install.packages('PCGSE')"
RUN R --no-save -e "install.packages('BiocManager')"
RUN R --no-save -e "install.packages('circilize')"
RUN R --no-save -e "install.packages('VennDiagram')"
RUN R --no-save -e "install.packages('ggthemes')"
RUN R --no-save -e "install.packages('Signac')"
RUN R --no-save -e "install.packages('lsmeans')"
RUN R --no-save -e "install.packages('rafalib')"
RUN R --no-save -e "BiocManager::install('monocle')"
RUN R --no-save -e "BiocManager::install('GEOquery')"
RUN R --no-save -e "BiocManager::install('fgsea')"
RUN R --no-save -e "BiocManager::install('MLP')"
RUN R --no-save -e "BiocManager::install('SingleCellExperiment')"
RUN R --no-save -e "BiocManager::install('muscat')"
RUN R --no-save -e "BiocManager::install('multtest')"
RUN R --no-save -e "BiocManager::install('SingleR')"
RUN R --no-save -e "BiocManager::install('scran')"
RUN R --no-save -e "BiocManager::install('Biobase')"
RUN R --no-save -e "BiocManager::install('scRNAseq')"
RUN R --no-save -e "BiocManager::install('esetVis')"
RUN R --no-save -e "BiocManager::install('limma')"
RUN R --no-save -e "BiocManager::install('edgeR')"
RUN R --no-save -e "BiocManager::install('DESeq2')"
RUN R --no-save -e "BiocManager::install('scater')"
RUN R --no-save -e "BiocManager::install('MAST')"
RUN R --no-save -e "BiocManager::install('zinbwave')"
RUN R --no-save -e "BiocManager::install('ComplexHeatmap')"
RUN R --no-save -e "BiocManager::install('LoomExperiment')"
RUN R --no-save -e "BiocManager::install('scmap')"
RUN R --no-save -e "BiocManager::install('MOFA2')"
RUN R --no-save -e "BiocManager::install('schex')"
RUN R --no-save -e "BiocManager::install('DropletUtils')"
RUN R --no-save -e "BiocManager::install('scDblFinder')"
RUN R --no-save -e "BiocManager::install('RUVSeq')"
RUN R --no-save -e "install.packages('devtools')"
RUN R --no-save -e "devtools::install_github('immunogenomics/harmony')"
RUN R --no-save -e "devtools::install_github('cellgeni/sceasy')"
RUN R --no-save -e "devtools::install_github('satijalab/seurat-wrappers')"
RUN R --no-save -e "devtools::install_github('mojaveazure/loomR')"
RUN R --no-save -e "devtools::install_github('MarioniLab/miloR', ref='devel')"
RUN R --no-save -e "devtools::install_github('Sun-lab/ideas')"
RUN R --no-save -e "devtools::install_github('heiniglab/scPower')"
RUN R --no-save -e "devtools::install_github('neurorestore/Libra')"
RUN R --no-save -e "devtools::install_github('PaulingLiu/ROGUE')"
RUN R --no-save -e "devtools::install_github('WilsonImmunologyLab/LinQView')"
RUN R --no-save -e "devtools::install_github('satijalab/seurat-data')"
RUN R --no-save -e "devtools::install_github('koenvandenberge/dayjob')"
RUN R --no-save -e "remotes::install_github('carmonalab/UCell')"
RUN R --no-save -e "remotes::install_github('carmonalab/scGate')"
RUN R --no-save -e "remotes::install_github('carmonalab/ProjecTILs')"
USER ubuntu
