import { TestBed, inject } from '@angular/core/testing';

import { ScopeModelService } from './scope-model.service';

describe('ScopeModelService', () => {
  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [ScopeModelService]
    });
  });

  it('should be created', inject([ScopeModelService], (service: ScopeModelService) => {
    expect(service).toBeTruthy();
  }));
});
